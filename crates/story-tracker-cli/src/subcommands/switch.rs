use git2::{BranchType, Repository};
use pivotal_tracker::client::Client;
use std::error::Error;

#[derive(Debug)]
pub struct RunOptions<'a> {
	pub branch_or_story_id: &'a String,
	pub client: &'a Client,
}

pub async fn run(_options: RunOptions<'_>) -> Result<(), Box<dyn Error>> {
	println!("{}", get_default_branch().unwrap());

	Ok(())
}

fn get_default_branch() -> Option<String> {
	let repo = Repository::open_from_env().unwrap();
	let revspec = repo.revparse_single("refs/remotes/origin/HEAD").unwrap();

	println!("Revspec: {:?}", revspec);

	let commit = revspec.as_commit().unwrap();
	let commit_id = commit.id();

	println!("Commit ID: {}", commit_id);

	let branches = repo.branches(Some(BranchType::Remote)).unwrap();
	let branch_name = branches
		.filter_map(|branch_tuple| {
			let (branch, _) = branch_tuple.unwrap();
			let branch_name = branch.name().unwrap().unwrap().to_string();

			if branch_name == "origin/HEAD" {
				return None;
			}

			let branch_commit = branch.get().peel_to_commit().unwrap();
			if branch_commit.id() == commit_id {
				Some(branch_name)
			} else {
				None
			}
		})
		.next()?;
	Some(branch_name.to_string())

	// let mut remote = repo.find_remote("origin").unwrap();
	// let mut remote_callbacks = RemoteCallbacks::new();

	// remote_callbacks.credentials(|_url, username_from_url, _allowed_types| {
	// 	// git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
	// 	git2::Cred::ssh_key(
	// 		username_from_url.unwrap_or("git"),
	// 		None,
	// 		std::path::Path::new("/Users/jondpenton/.ssh/id_ed25519"),
	// 		None,
	// 	)
	// });

	// println!("Connecting to origin...");

	// remote
	// 	.connect_auth(Direction::Fetch, Some(remote_callbacks), None)
	// 	.unwrap();

	// println!("Connected to origin. Getting default branch...");

	// let default_branch = remote.default_branch().unwrap();

	// println!("Got default branch. Disconnecting...");

	// remote.disconnect().unwrap();

	// println!("Disconnected.");

	// default_branch.as_str().unwrap().to_string()
}
