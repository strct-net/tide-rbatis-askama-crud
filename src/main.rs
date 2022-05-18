mod app;
mod handlers;
mod models;

use fast_log::Config;
use log::info;

#[async_std::main]
async fn main() -> Result<(), tide::Error> {
    dotenv::dotenv()?;

    fast_log::init(Config::new().console())?;

    app::init_db().await?;

    let bind = "127.0.0.1:3030";

    info!("Running server at: {}", bind);
    let app = app::init_server();
    app.listen(bind).await?;

    Ok(())
}
