use askama::Template;

use rbatis::crud_enable;
#[crud_enable]
#[derive(Debug)]
pub struct Post {
    pub id: Option<String>,
    pub content: Option<String>,
}

#[derive(Template)]
#[template(path = "list_posts.html")]
pub struct ListPostsTemplate {
    pub posts: Option<Vec<Post>>,
}
