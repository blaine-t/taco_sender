# Taco Sender

A Rust application that sends scheduled Slack messages with taco emojis and user mentions.

## Setup

1. Copy the example environment file:

   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your actual values:
    - `SLACK_BASE_URL`: Your Slack organizations base URL (starts with `https://`)
    - `SLACK_CHANNEL_ID`: The ID of the Slack channel where you want to send the message
    - `SLACK_COOKIE`: Your Slack session cookie (copy from browser dev tools)
    - `SLACK_TOKEN`: Your Slack API token (starts with `xoxc-`)
    - `SLACK_USER_IDS`: Comma-separated list of Slack user IDs to mention

## Usage

Run the application:

```bash
cargo run
```

Or build and run the release version:

```bash
cargo run --release
```

## Environment Variables

The application supports the following environment variables:

- `SLACK_BASE_URL`: Your Slack organizations base URL
- `SLACK_CHANNEL_ID`: Target channel ID
- `SLACK_COOKIE`: Your Slack session cookie
- `SLACK_TOKEN`: Your Slack API token
- `SLACK_USER_IDS`: Comma-separated user IDs to mention

If any environment variable is not set, the application will bail and you will need to provide them.

## Getting Slack Credentials

### Token

You can get your Slack token from the browser's dev tools:

1. Open Slack in your browser
2. Open Developer Tools (F12)
3. Go to Network tab
4. Make a request in Slack
5. Look for requests to `slack.com/api/`
6. Check the form data for the `token` field

### Cookie

1. Open Slack in your browser
2. Open Developer Tools (F12)
3. Go to Application/Storage tab
4. Look for cookies under `slack.com`
5. Copy the entire cookie string

### Channel ID

1. Open the Slack channel
2. Look at the URL - the channel ID is at the end (starts with `C`)

### User IDs

1. Right-click on a user's profile
2. Copy their member ID (starts with `U`)
