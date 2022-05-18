use askama::Template;

use rbatis::crud_table;
use serde::{Deserialize, Serialize};

#[crud_table]
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<String>,
    pub content: Option<String>,
}

#[derive(Template)]
#[template(path = "list_posts.html")]
pub struct ListPostsTemplate {
    pub posts: Option<Vec<Post>>,
}
