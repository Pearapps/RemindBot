# What is Remindbot

Remindbot is a github bot that reminds github assignees of stale pull requests.

# How it works

Remindbot works by looking at all the pull requests in a repo and checking to see if any of the comments are:

1. By the current assignee 
2. Within the last day (the time is configurable)

If these two are not true and the PR is older than a day (also configurable, matches the time specified in #2 above), the bot will leave a comment on the Pull Request to remind the assignee that they need to review.

# How to use:

1. Compile the bot
	- Use `cargo build --release`
2. cd to the directory where the binary is built (by default its at ./target/release)
3. The command to run remind bot is `remindbot --owner pearapps --repo initializeme --auth_token SOME_TOKEN` 

# What else

This bot also will tell you the average amount of time all open Pull Requests with an assignee have been open.
