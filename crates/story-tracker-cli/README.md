# Story Tracker CLI

CLI tool for generating story branch names and switching to them. Currently only supports Pivotal Tracker.

## Providers

### Pivotal Tracker

You will need to grab the API token from [your profile](https://www.pivotaltracker.com/profile) and set it to the environment variable `PIVOTAL_TRACKER_TOKEN`.

## Commands

Copy the link of your story, the story's ID, or the branch's name. The following formats are currently supported:

- https://www.pivotaltracker.com/story/show/12345678
- https://www.pivotaltracker.com/n/projects/12345678/stories/12345678
- #12345678
- 12345678
- feature/story-name-#12345678

Then use it like so in the following subcommands:

### stb generate

```bash
stb generate https://www.pivotaltracker.com/story/show/12345678
stb gen https://www.pivotaltracker.com/story/show/12345678
```

#### Example

```bash
$ stb generate https://www.pivotaltracker.com/story/show/12345678
Getting story...
Generating branch name...
feature/story-name-#185916370
```

### stb switch

```bash
stb switch https://www.pivotaltracker.com/story/show/12345678
```

#### Example

```bash
$ stb switch https://www.pivotaltracker.com/story/show/12345678
Fetching story 12345678...
Fetching remote branch origin/feature/story-name-#12345678...
Checking out branch master...
Pulling latest from master...
Creating branch feature/story-name-#12345678...
```

### Additional Info

The statuses shown by the tool are sent to stderr, while only the branch name is sent to stdout. This means you can use this tool in combination with other commands like:

```bash
# Copy to clipboard
$ stb gen https://www.pivotaltracker.com/story/show/12345678 | pbcopy
```

## Configuration

### Environment Variables

#### `PIVOTAL_TRACKER_TOKEN`

This will need to reflect the token on your profile page, as seen in the [Pivotal Tracker](#pivotal-tracker) section.
