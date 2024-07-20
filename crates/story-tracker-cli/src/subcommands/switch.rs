use crate::subcommands::generate::branch_name;
use auth_git2::GitAuthenticator;
use git2::{BranchType, Repository};
use pivotal_tracker::{
	client::Client,
	story::{GetStoryOptions, StoryID},
};
use std::error::Error;

#[derive(Debug)]
pub struct RunOptions<'a> {
	pub branch_or_story_id: &'a String,
	pub client: &'a Client,
}

pub async fn run(options: RunOptions<'_>) -> Result<(), Box<dyn Error>> {
	// println!("{}", get_default_branch());

	let branch_name = {
		let story_id = options.branch_or_story_id.parse::<StoryID>();

		if story_id.is_ok() {
			let story = options
				.client
				.get_story(GetStoryOptions {
					id: story_id.unwrap(),
				})
				.await
				.expect("Failed to get story");

			branch_name(&story)
		} else {
			options.branch_or_story_id.to_string()
		}
	};

	println!("{}", branch_name);
	// println!("Running 'git fetch --all'...");

	let auth = GitAuthenticator::default();
	let repo = Repository::open_from_env()?;
	let mut remote = repo.find_remote("origin")?;

	remote
		.fetch_refspecs()?
		.iter()
		.for_each(|x| auth.fetch(&repo, &mut remote, &[x.unwrap()], None).unwrap());

	println!(
		"{}",
		repo
			.branch_upstream_name(remote.default_branch()?.as_str().unwrap(),)?
			.as_str()
			.unwrap()
	);

	Ok(())
}

#[allow(dead_code)]
fn get_default_branch() -> String {
	let repo = Repository::open_from_env().unwrap();
	let revspec = repo.revparse_single("refs/remotes/origin/HEAD").unwrap();
	let commit = revspec.as_commit().unwrap();
	let commit_id = commit.id();
	let mut branches = repo.branches(Some(BranchType::Remote)).unwrap();
	let branch_name = branches
		.find_map(|branch_tuple| {
			let (branch, _) = branch_tuple.unwrap();
			let branch_name = branch.name().unwrap().unwrap().to_string();

			if branch_name == "origin/HEAD" {
				return None;
			}

			let branch_commit = branch.get().peel_to_commit().unwrap();

			if branch_commit.id() != commit_id {
				return None;
			}

			Some(branch_name)
		})
		.unwrap();

	branch_name.to_string()
}
