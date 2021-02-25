use tide::Response;
pub mod post;

// pub async fn index(_req: tide::Request<()>) -> tide::Result {
//     let index = YourIndexTemplate {};
//     Ok(index.into())
// }

pub async fn css(_req: tide::Request<()>) -> tide::Result {
    let mut res = Response::new(200);
    res.set_body(include_str!("../templates/style.css"));
    res.set_content_type("text/css");

    Ok(res)
}
