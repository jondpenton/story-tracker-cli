use std::error::Error;

use pivotal_tracker::{
  client::Client,
  story::{GetStoryOptions, Story, StoryID},
};
use slug::slugify;

pub struct RunOptions<'a> {
  pub client: &'a Client,
  pub story_id: &'a String,
}

pub async fn run(options: RunOptions<'_>) -> Result<(), Box<dyn Error>> {
  let RunOptions { story_id, client } = options;
  let story_id = story_id.parse::<StoryID>()?;

  println!("Getting story...");

  let story = client.get_story(GetStoryOptions { id: story_id }).await?;

  println!("Generating branch name...");

  let branch_name = branch_name(&story);

  println!("{:#?}", branch_name);

  Ok(())
}

fn branch_name(story: &Story) -> String {
  let story_name_slug = slugify(story.name.trim());
  let mut story_name_words = story_name_slug.split("-").collect::<Vec<_>>();
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

  branch_name_parts.join("-")
}
