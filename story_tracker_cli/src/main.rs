use std::{env, error};

use pivotal_tracker::{
  client::{Client, ClientNewOptions},
  story::{GetStoryOptions, StoryID},
};
use slug::slugify;

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
  let story_name_slug = slugify(story.name.trim());
  let mut story_name_words = story_name_slug.split("-").collect::<Vec<&str>>();
  let branch_name_parts = vec![format!(
    "{}/{}",
    String::from(story.story_type),
    story_name_words.remove(0)
  )];

  println!(
    "{:?} {:?} {:?}",
    branch_name_parts, story_name_words, story.id.0
  );

  Ok(())
}
