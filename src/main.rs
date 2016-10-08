mod models;
mod networking;

#[macro_use]
extern crate hyper;
extern crate rustc_serialize;
extern crate chrono;
extern crate argparse;

header! { (UserAgent, "User-Agent") => [String] }
header! { (Authorization, "Authorization") => [String] }

use hyper::client::Client;
use hyper::header::Headers;
use chrono::*;
use argparse::{ArgumentParser, Store};

use models::*;
use networking::*;

fn main() {

    let mut repo_owner = String::from("");
    let mut repo = String::from("");
    let mut time_since_comment_to_consider_disqualification = 24 * 60 * 60;
    let mut message = "Bump".to_string();
    let mut authorization_token = String::from("");

    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut repo_owner)
            .add_option(&["--owner"], Store, "Owner of the github repo.");
        parser.refer(&mut repo)
            .add_option(&["--repo"], Store, "The name of the github repo.");

        parser.refer(&mut time_since_comment_to_consider_disqualification)
            .add_option(&["--seconds"],
                          Store,
                          "The lower limit of seconds since the last time the assignee has 
   					  \
                           commented on a PR to remind the assignee via a comment.");

        parser.refer(&mut message)
            .add_option(&["--message"],
                          Store,
                          "The message in the comment on the open pull request.
   					  The \
                           default is 24 * 60 * 60; which is a day in seconds.");

        parser.refer(&mut authorization_token)
            .add_option(&["--auth_token"],
                          Store,
                          "The personal access token of the user that will be requesting \
                           information on the repo's pull requests
   					  and commenting on \
                           the pull requests if needed.");

        parser.parse_args_or_exit();
    }

    if repo_owner.len() == 0 || repo.len() == 0 || authorization_token.len() == 0 {
        panic!("Invalid input, you must give a owner, repo and auth_token");
    }

    let url = &format!("https://api.github.com/repos/{}/{}/pulls", repo_owner, repo);

    let dt = Local::now();

    let http_client = Client::new();

    let mut headers = Headers::new();
    headers.set(UserAgent("remind-bot".to_string()));
    headers.set(Authorization(format!("token {}", authorization_token).to_string()));

    let headers_clone = headers.clone();

    let user: Option<User> = get_model_from_network("https://api.github.com/user", &http_client, headers.clone());

    if user.is_none() {
    	println!("Could not get user information");
    };

    let requests: Vec<PullRequest> = get_models_from_network(url, &http_client, headers);

    let pull_requests: Vec<&PullRequest> = 
	requests
	.iter()
	.map(|request| {

		let new_headers = headers_clone.clone();

		let mut comments = get_models_from_network(&request._links.comments.href, &http_client, new_headers.clone());

		comments.extend(get_models_from_network(&request._links.review_comments.href, &http_client, new_headers));

		return (request, comments);
	})
	.map(|x| {

		// This code block is responsbible for filtering out comments that are not by the assignee.

		if let Some(ref assignee) = x.0.assignee {
			let comments: Vec<Comment> = 
			x.1
			.iter()
			.filter(|x: &&Comment| {
				if let &Some(ref current_user) = &user {
					println!("{:?}", current_user);

					return x.user.login == current_user.login || x.user.login == assignee.login;
				}
				else {
					x.user.login == assignee.login
				}
			})
			.cloned()
			.collect();

			return (x.0, comments);
		}

		return x;
	})
	.filter(|x| {

		let pull_request_time: Result<DateTime<FixedOffset>, ParseError> = x.0.created_at_date_time();

		if let Some(pull_request_date_time) = pull_request_time.ok() {
			let comments: Vec<&Comment> = x.1.iter().filter(|x: &&Comment| {

				let time: Result<DateTime<FixedOffset>, ParseError> = x.created_at_date_time();

				if let Some(date_time) = time.ok() {
					return !is_after_desired_time(&date_time, &dt, time_since_comment_to_consider_disqualification);
				}

				return false
			}).collect();

			return
			comments.len() == 0
			|| 
			// We want to check the length of the unfiltered array here because this is checking if 
			// the pull requet has no comments by the assignee at all and is older than the specified time.
			// This works because in the previous filter filters the comments to ones created by the assignee.
			(x.1.len() == 0 && is_after_desired_time(&pull_request_date_time, &dt, time_since_comment_to_consider_disqualification));
		}

		return false
	})
	.map(|x|
		// We just want the pull requests at this point
		x.0
	)
	.collect();

    for pull_request in &pull_requests {
        if let Some(ref assignee) = pull_request.assignee {
            let mut final_message_string = format!("@{} ", assignee.login);
            final_message_string.push_str(&message);
            post_request(&pull_request._links.comments.href,
                         &http_client,
                         headers_clone.clone(),
                         &final_message_string);
        }

    }

    let to_measure = 
    requests
    .iter()
    .filter(|x| {
    	match x.assignee {
    		Some(_) => true,
    		_ => false
    	}
    })
    .map(|pr| {
		if let Some(time) = pr.created_at_date_time().ok() {
			return Some(dt.timestamp() - time.timestamp());
		}

    	None
    })
    .filter(|pr| {
    	pr.is_some()
    })
    .fold(0, |x, y| {

    	if let Some(second) = y {
    		return x + second;
    	}

    	x

    }) / (requests.len() as i64);

    println!("The average time the currently open pull requests with assignees have been open in {:?} is {:?} seconds", repo, to_measure);
}

fn is_after_desired_time(date_time: &DateTime<FixedOffset>, 
	desired_time: &DateTime<Local>,
	seconds: i64) -> bool {
		desired_time.timestamp() - date_time.timestamp() > seconds
}
