use std::{env, error::Error};

use clap::{Parser, Subcommand};
use pivotal_tracker::client::{Client, ClientNewOptions};
use story_tracker_cli::subcommands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CLI {
  #[clap(subcommand)]
  command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
  #[clap(alias = "gen")]
  Generate { story_id: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let client = Client::new(ClientNewOptions {
    api_key: env::var("PIVOTAL_TRACKER_API_KEY")?,
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
    None => Ok(()),
  }
}
