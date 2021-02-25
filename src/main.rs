mod handlers;
mod models;
use handlers::{
    css,
    post::{add_post, delete_post, edit_post, list_all_posts, view_post},
};
use rbatis::rbatis::Rbatis;

use barrel::backend::Pg;
use barrel::{types, Migration};

use lazy_static::lazy_static;
use std::env;

// Initialize the global Rbatis variable.
lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[async_std::main]
async fn main() -> Result<(), tide::Error> {
    dotenv::dotenv()?;

    fast_log::init_log("tide.log", 1000, log::Level::Info, None, true)?;

    // Initial Migration
    let mut m = Migration::new();
    m.create_table_if_not_exists("post", |t| {
        t.add_column("id", types::varchar(255));
        t.add_column("content", types::varchar(255));
    });

    let migration = &m.make::<Pg>();

    let db_url = env::var("DATABASE_URL").expect("Missing 'DATABASE_URL' Environment variable.");

    // Connect Rbatis to URL provided in .env file. Don't forget to create it! (Or, well, use normal envvars.)
    RB.link(&db_url).await?;

    RB.exec("", migration).await?;

    let mut app = tide::new();

    app.at("/").serve_file("templates/hello_index.html")?;
    app.at("/style.css").get(css);
    app.at("/:id").get(view_post);
    app.at("/posts").get(list_all_posts);
    app.at("/post").post(add_post);
    app.at("/post").patch(edit_post);
    app.at("/post").delete(delete_post);

    app.listen("127.0.0.1:3030").await?;

    Ok(())
}
