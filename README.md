# WKW -- Wer kommt

## Was?

Wer kommt wann Slack-App um zu sagen, wer wann ins Büro kommt

## Installation in Slack

- go to <https://api.slack.com/apps>
- click on 'create new app'
- 'from scratch'
- 'Slash Commands'
  - 'Create New Command'
  - set name
  - Request URL: Lambda-Url + Prefix + /init
- 'Interactivity & Shortcuts'
  - Request-URL: Lambda-Url + Prefix
- Set icon in basic information (bottom)
- 'Install App'

## Dependencies

### cargo make

This project uses `cargo make`, install it with `cargo install cargo-make`

### available tasks

- `cargo make build-lambda`: builds the lambda function artifacts
- `cargo make deploy`: builds and deploys the lambda function artifacts
