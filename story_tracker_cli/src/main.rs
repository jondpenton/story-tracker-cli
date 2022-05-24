use std::{env, error};

use pivotal_tracker::{
  client::{Client, ClientNewOptions},
  story::{GetStoryOptions, StoryID},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  let client = Client::new(ClientNewOptions {
    api_key: env::var("PIVOTAL_TRACKER_API_KEY")?,
    api_version: 5,
  });
  let story = client
    .get_story(GetStoryOptions {
      id: StoryID(182171003),
    })
    .await?;

  println!("{:?}", story);

  Ok(())
}
