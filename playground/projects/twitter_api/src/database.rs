use crate::models::Post;

#[derive(Clone, Debug)]
pub struct Database
{
	posts: Vec<Post>
}

impl Database
{
	pub fn new() -> Database { Database { posts: vec![] } }

	pub fn add_post(&mut self, new_post: Post) { self.posts.push(new_post) }

	pub fn get_posts(&self) -> &Vec<Post> { &self.posts }
}
