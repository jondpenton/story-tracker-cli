use std::{env, error};

use pivotal_tracker::{
  client::{Client, ClientNewOptions},
  story::{GetStoryOptions, StoryID},
};
use story_tracker_cli::generate::branch_name;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  let client = Client::new(ClientNewOptions {
    api_key: env::var("PIVOTAL_TRACKER_API_KEY")?,
    api_version: 5,
  });
  let story = client
    .get_story(GetStoryOptions {
      id: StoryID(181978572),
    })
    .await?;

  println!("{}", branch_name(&story));

  Ok(())
}
