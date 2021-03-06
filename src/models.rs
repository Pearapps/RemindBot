use chrono::*;

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct PullRequest {
    pub number: i32,
    pub created_at: String,
    pub assignee: Option<User>,
    pub assignees: Option<Vec<User>>,
    pub _links: Links,
}

impl PullRequest {
	pub fn assignees(&self) -> Option<Vec<User>>{
		if let Some(ref assignees) = self.assignees {
			return Some(assignees.clone());
		} 
		else if let Some(ref assignee) = self.assignee {
			return Some([assignee.clone()].to_vec());
		}
		else {
			None
		}
	}
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct Comment {
    pub user: User,
    pub created_at: String,
}

impl Comment {
    pub fn created_at_date_time(&self) -> Result<DateTime<FixedOffset>, ParseError> {
        return DateTime::parse_from_rfc3339(&self.created_at);
    }
}

impl PullRequest {
    pub fn created_at_date_time(&self) -> Result<DateTime<FixedOffset>, ParseError> {
        return DateTime::parse_from_rfc3339(&self.created_at);
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct User {
    pub login: String,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct Links {
    pub comments: Link,
    pub review_comments: Link,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct Link {
    pub href: String,
}
