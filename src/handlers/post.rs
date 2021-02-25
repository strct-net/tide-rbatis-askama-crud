use crate::models::{ListPostsTemplate, Post};
use crate::RB;
use rbatis::crud::CRUD;
use serde_json::json;
use tide::Response;

pub async fn add_post(mut req: tide::Request<()>) -> tide::Result {
    use nanoid::nanoid;
    // Get request body as valid Post, else return error
    let mut post: Post = req.body_json().await?;

    // Define an alphabet for nicer looking IDs
    let alphabet: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    // Generate a (probably) unique ID for the new post
    post.id = Some(nanoid!(10, &alphabet));

    RB.save("", &post).await?;

    Ok(json!(post).into())
}

pub async fn list_all_posts(_req: tide::Request<()>) -> tide::Result {
    let posts = RB.fetch_list::<Post>("").await?;
    let res = ListPostsTemplate { posts: Some(posts) };

    Ok(res.into())
}

pub async fn view_post(req: tide::Request<()>) -> tide::Result {
    // Gets the "/:id" part of our URL, e.g. "localhost:3030/hello"
    // would set the post_id to "hello"
    let post_id = req.param("id")?;
    match RB.fetch_by_id::<Post>("", &post_id.to_string()).await {
        Ok(post) => Ok(json!(post).into()),
        // Error if no rows are returned (no posts found with that ID)
        Err(_) => Ok(Response::new(404)),
    }
}

pub async fn edit_post(mut req: tide::Request<()>) -> tide::Result {
    let mut post: Post = req.body_json().await?;

    RB.update_by_id::<Post>("", &mut post).await?;
    Ok(json!(post).into())
}

pub async fn delete_post(mut req: tide::Request<()>) -> tide::Result {
    let post: Post = req.body_json().await?;
    let mut res = Response::new(200);

    if let Some(id) = post.id {
        RB.remove_by_id::<Post>("", &id).await?;
    } else {
        // Return error if no ID is given.
        res.set_status(400);
        res.set_body("no id");
    };
    Ok(res)
}
