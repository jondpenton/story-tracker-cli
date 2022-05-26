use pivotal_tracker::story::Story;
use slug::slugify;

pub fn branch_name(story: &Story) -> String {
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

  branch_name_parts.join("-")
}