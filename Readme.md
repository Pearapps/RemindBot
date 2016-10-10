# What is Remindbot?

Remindbot is a GitHub bot that reminds github assignees of stale pull requests.

# How it works

Remindbot works by looking at all the pull requests in a repository and checking to see if any of the comments are:

* By the current assignee 
* Within a certain time (by default, the last day)

If these two are not true and the PR is older than a day (also configurable), the bot will leave a comment on the Pull Request to remind the assignee that they need to review it.

# How to use

1. Compile the bot using [Cargo](https://crates.io)
	- Use `cargo build --release`
2. `cd` to the directory where the binary is built (by default its at `./target/release`)
3. The command to run RemindBot is `remindbot --owner pearapps --repo initializeme --auth_token SOME_TOKEN` 
	- `--owner` is the GitHub user whose repo you want to remind assignees on
	- `--repo` is the repo name you want to remind assignees on
	- This will run RemindBot once
	- If you want RemindBot to run continuously - you have to handle that yourself for now.

# What else

This bot also will tell you the average amount of time all open Pull Requests with an assignee have been open.

## Why rust?

Rust is expressive, safe, and fast.
