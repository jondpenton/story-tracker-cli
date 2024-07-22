use clap::{Parser, Subcommand};
use pivotal_tracker::client::{Client, ClientNewOptions};
use std::{env, error::Error};
use story_tracker_cli::subcommands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
	#[command(subcommand)]
	command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
	#[command(alias = "gen")]
	Generate {
		story_id: String,
	},
	Switch {
		branch_or_story_id: String,
	},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let client = Client::new(ClientNewOptions {
		api_version: 5,
		token: env::var("PIVOTAL_TRACKER_TOKEN")
			.expect("Environment variable PIVOTAL_TRACKER_TOKEN is required"),
	});
	let cli = CLI::parse();

	match &cli.command {
		Some(Command::Generate { story_id }) => {
			subcommands::generate::run(subcommands::generate::RunOptions {
				client: &client,
				story_id,
			})
			.await
		}
		Some(Command::Switch { branch_or_story_id }) => {
			subcommands::switch::run(subcommands::switch::RunOptions {
				client: &client,
				branch_or_story_id,
			})
			.await
		}
		None => Ok(()),
	}
}

#[cfg(test)]
mod tests {
	use assert_cmd::Command;
	use pivotal_tracker::story::StoryID;

	const BRANCH_NAME: &str = "feature/create-config-subcommand-#185916371";
	const STORY_ID: StoryID = StoryID(185916371);
	const STORY_PAGE_LINK: &str =
		"https://www.pivotaltracker.com/n/projects/2673618/stories/185916371";
	const STORY_TILE_LINK: &str =
		"https://www.pivotaltracker.com/story/show/185916371";

	#[test]
	fn test_generate_pound_story_id() {
		let mut cmd = Command::cargo_bin("stb").unwrap();
		let assert = cmd
			.arg("generate")
			.arg(format!("#{}", STORY_ID))
			.env("exit", "0")
			.assert();

		assert
			.success()
			.code(0)
			.stdout(format!("{}\n", BRANCH_NAME));
	}

	#[test]
	fn test_generate_story_id() {
		let mut cmd = Command::cargo_bin("stb").unwrap();
		let assert = cmd
			.arg("generate")
			.arg(STORY_ID.to_string())
			.env("exit", "0")
			.assert();

		assert
			.success()
			.code(0)
			.stdout(format!("{}\n", BRANCH_NAME));
	}

	#[test]
	fn test_generate_story_page_link() {
		let mut cmd = Command::cargo_bin("stb").unwrap();
		let assert = cmd
			.arg("generate")
			.arg(STORY_PAGE_LINK)
			.env("exit", "0")
			.assert();

		assert
			.success()
			.code(0)
			.stdout(format!("{}\n", BRANCH_NAME));
	}

	#[test]
	fn test_generate_story_tile_link() {
		let mut cmd = Command::cargo_bin("stb").unwrap();
		let assert = cmd
			.arg("generate")
			.arg(STORY_TILE_LINK)
			.env("exit", "0")
			.assert();

		assert
			.success()
			.code(0)
			.stdout(format!("{}\n", BRANCH_NAME));
	}

	#[test]
	fn test_gen_pound_story_id() {
		let mut cmd = Command::cargo_bin("stb").unwrap();
		let assert = cmd
			.arg("gen")
			.arg(format!("#{}", STORY_ID))
			.env("exit", "0")
			.assert();

		assert
			.success()
			.code(0)
			.stdout(format!("{}\n", BRANCH_NAME));
	}

	#[test]
	fn test_gen_story_id() {
		let mut cmd = Command::cargo_bin("stb").unwrap();
		let assert = cmd
			.arg("gen")
			.arg(STORY_ID.to_string())
			.env("exit", "0")
			.assert();

		assert
			.success()
			.code(0)
			.stdout(format!("{}\n", BRANCH_NAME));
	}

	#[test]
	fn test_gen_story_page_link() {
		let mut cmd = Command::cargo_bin("stb").unwrap();
		let assert = cmd
			.arg("gen")
			.arg(STORY_PAGE_LINK)
			.env("exit", "0")
			.assert();

		assert
			.success()
			.code(0)
			.stdout(format!("{}\n", BRANCH_NAME));
	}

	#[test]
	fn test_gen_story_tile_link() {
		let mut cmd = Command::cargo_bin("stb").unwrap();
		let assert = cmd
			.arg("gen")
			.arg(STORY_TILE_LINK)
			.env("exit", "0")
			.assert();

		assert
			.success()
			.code(0)
			.stdout(format!("{}\n", BRANCH_NAME));
	}
}
