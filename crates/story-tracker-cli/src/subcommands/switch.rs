use crate::subcommands::generate::branch_name;
use auth_git2::GitAuthenticator;
use git2::{BranchType, Repository};
use pivotal_tracker::{
	client::Client,
	story::{GetStoryOptions, StoryID},
};
use std::{error::Error, process::Command};

#[derive(Debug)]
pub struct RunOptions<'a> {
	pub branch_or_story_id: &'a String,
	pub client: &'a Client,
}

pub async fn run(options: RunOptions<'_>) -> Result<(), Box<dyn Error>> {
	let branch_name = {
		let story_id = options.branch_or_story_id.parse::<StoryID>();

		if story_id.is_ok() {
			let story_id = story_id?;

			println!("Fetching story {}...", story_id.0);

			let story = options
				.client
				.get_story(GetStoryOptions { id: story_id })
				.await
				.expect("Failed to get story");

			branch_name(&story)
		} else {
			options.branch_or_story_id.to_string()
		}
	};

	println!("Fetching remote branch origin/{}...", branch_name);

	let auth = GitAuthenticator::default();
	let repo = Repository::open_from_env()?;
	let mut remote = repo.find_remote("origin")?;

	remote
		.fetch_refspecs()?
		.iter()
		.for_each(|x| auth.fetch(&repo, &mut remote, &[x.unwrap()], None).unwrap());

	let remote_has_branch = repo
		.find_branch(&format!("origin/{}", branch_name), BranchType::Remote)
		.is_ok();

	if remote_has_branch {
		checkout_branch(&branch_name);
	} else {
		branch_off_of(&get_default_branch(), &branch_name);
	}

	Ok(())
}

fn checkout_branch(branch_name: &str) {
	println!("Checking out branch {}...", branch_name);

	Command::new("git")
		.args(["checkout", branch_name])
		.output()
		.unwrap();

	println!("Pulling latest from {}...", branch_name);

	Command::new("git").args(["pull"]).output().unwrap();
}

fn branch_off_of(from_branch_name: &str, to_branch_name: &str) {
	checkout_branch(from_branch_name);

	println!("Creating branch {}...", to_branch_name);

	Command::new("git")
		.args(["checkout", "-b", to_branch_name])
		.output()
		.unwrap();
}

fn get_default_branch() -> String {
	let repo = Repository::open_from_env().unwrap();
	let origin_revspec =
		repo.revparse_single("refs/remotes/origin/HEAD").unwrap();
	let origin_commit = origin_revspec.as_commit().unwrap();
	let origin_commit_id = origin_commit.id();
	let mut branches = repo.branches(Some(BranchType::Remote)).unwrap();
	let origin_branch_name = branches
		.find_map(|branch_tuple| {
			let (branch, _) = branch_tuple.unwrap();
			let branch_name = branch.name().unwrap().unwrap().to_string();

			if branch_name == "origin/HEAD" {
				return None;
			}

			let branch_commit = branch.get().peel_to_commit().unwrap();

			if branch_commit.id() != origin_commit_id {
				return None;
			}

			Some(branch_name)
		})
		.unwrap();

	origin_branch_name.replace("origin/", "")
}
