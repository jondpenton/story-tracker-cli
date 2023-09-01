use git2::{BranchType, Repository};
use pivotal_tracker::client::Client;
use std::error::Error;

#[derive(Debug)]
pub struct RunOptions<'a> {
	pub branch_or_story_id: &'a String,
	pub client: &'a Client,
}

pub async fn run(_options: RunOptions<'_>) -> Result<(), Box<dyn Error>> {
	println!("{}", get_default_branch());

	Ok(())
}

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
