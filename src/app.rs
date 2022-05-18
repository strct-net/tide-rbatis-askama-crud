use std::env;

use barrel::{backend::Pg, types, Migration};
use log::info;
use rbatis::rbatis::Rbatis;
use rbson::Bson;

use crate::handlers::{
    css,
    post::{add_post, delete_post, edit_post, list_all_posts, view_post},
};

pub fn init_server() -> tide::Server<()> {
    let mut app = tide::new();

    app.at("/")
        .serve_file("templates/hello_index.html")
        .unwrap();
    app.at("/style.css").get(css);
    app.at("/posts").get(list_all_posts);
    app.at("/post").post(add_post);
    app.at("/post/:id").get(view_post);
    app.at("/post/:id").patch(edit_post);
    app.at("/post/:id").delete(delete_post);

    app
}

use lazy_static::lazy_static;

// Initialize the global Rbatis variable.
lazy_static! {
    pub static ref RB: Rbatis = Rbatis::new();
}

pub async fn init_db() -> Result<(), rbatis::Error> {
    // Initial Migration
    let mut m = Migration::new();
    m.create_table_if_not_exists("post", |t| {
        t.add_column("id", types::varchar(255));
        t.add_column("content", types::varchar(255));
    });

    let migration = &m.make::<Pg>();
    let db_url = env::var("DATABASE_URL").expect("Missing 'DATABASE_URL' Environment variable.");

    // Connect Rbatis to URL provided in the .env file,
    // or manually specify the DATABASE_URL environment variable.
    info!("Trying to connect to the database");
    RB.link(&db_url).await?;

    info!("Connected to database, applying migrations");
    RB.exec(migration, vec![Bson::Null]).await?;

    Ok(())
}
