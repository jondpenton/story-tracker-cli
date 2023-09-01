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
		api_key: env::var("PIVOTAL_TRACKER_API_KEY")
			.expect("Environment variable PIVOTAL_TRACKER_API_KEY is required"),
		api_version: 5,
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
