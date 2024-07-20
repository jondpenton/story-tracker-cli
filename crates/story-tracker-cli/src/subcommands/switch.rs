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
		checkout_branch(&repo, &branch_name);
	} else {
		let default_branch = get_default_branch();
		checkout_branch(&repo, &default_branch);

		println!("Creating branch {}...", branch_name);

		branch_off_of(&repo, &branch_name, &default_branch);
	}

	Ok(())
}

fn checkout_branch(repo: &Repository, branch_name: &str) {
	println!("Checking out branch {}...", branch_name);

	let remote_branch = repo
		.find_branch(&format!("origin/{}", branch_name), BranchType::Remote)
		.unwrap();
	let mut local_branch = repo
		.branch(
			branch_name,
			&remote_branch.get().peel_to_commit().unwrap(),
			false,
		)
		.unwrap_or_else(|_| {
			repo.find_branch(branch_name, BranchType::Local).unwrap()
		});

	local_branch
		.set_upstream(Some(&format!("origin/{}", branch_name)))
		.unwrap();

	repo
		.checkout_tree(local_branch.get().peel_to_tree().unwrap().as_object(), None)
		.expect("Failed to checkout");

	repo
		.set_head(&format!("refs/heads/{}", branch_name))
		.expect("Failed to set HEAD");

	let auth = GitAuthenticator::default();
	let mut remote = repo.find_remote("origin").unwrap();

	println!("Pulling latest from {}...", branch_name);

	auth
		.fetch(
			repo,
			&mut remote,
			&[&format!("refs/heads/{}", branch_name)],
			None,
		)
		.unwrap();

	let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
	let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap();

	local_branch
		.get_mut()
		.set_target(fetch_commit.id(), "Fast-Forward")
		.unwrap();
	repo
		.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
		.unwrap();
}

fn branch_off_of(
	repo: &Repository,
	to_branch_name: &str,
	from_branch_name: &str,
) {
	let from_branch = repo
		.find_branch(from_branch_name, BranchType::Local)
		.unwrap();
	let to_branch = repo
		.branch(
			to_branch_name,
			&from_branch.get().peel_to_commit().unwrap(),
			false,
		)
		.unwrap_or_else(|_| {
			repo.find_branch(to_branch_name, BranchType::Local).unwrap()
		});

	repo
		.checkout_tree(to_branch.get().peel_to_tree().unwrap().as_object(), None)
		.expect("Failed to checkout");

	repo
		.set_head(&format!("refs/heads/{}", to_branch_name))
		.expect("Failed to set HEAD");
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
