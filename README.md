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
- 'Install App'

## Dependencies

This project uses `cargo make`, install it with `cargo install cargo-make`
