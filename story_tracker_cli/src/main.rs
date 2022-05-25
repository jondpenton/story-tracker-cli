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
      id: StoryID(181978572),
    })
    .await?;
  let story_name_slug = slugify(story.name.trim());
  let mut story_name_words = story_name_slug.split("-").collect::<Vec<&str>>();
  let mut branch_name_parts = vec![
    format!(
      "{}/{}",
      String::from(story.story_type),
      story_name_words.remove(0)
    ),
    format!("#{}", story.id.0),
  ];

  loop {
    if story_name_words.len() == 0 {
      break;
    }

    let branch_length =
      branch_name_parts.iter().fold(0, |acc, x| acc + x.len())
        + branch_name_parts.len()
        - 1;

    if branch_length + story_name_words[0].len() + 1 > 50 {
      break;
    }

    branch_name_parts.insert(
      branch_name_parts.len() - 1,
      story_name_words.remove(0).to_string(),
    );
  }

  println!("{}", branch_name_parts.join("-"));

  Ok(())
}
