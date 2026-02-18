use dotenv::dotenv;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

fn header() {
    info!(r#"▄▄▄▄▄   ▄▄▄▄▄▄▄     ▄▄▄▄    ▄▄▄▄▄▄▄ ▄▄▄   ▄▄▄  ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄   "#);
    info!(r#"██▀▀██▄ ███▀▀███▄ ▄██▀▀██▄ ███▀▀▀▀▀ ███ ▄███▀ ███▀▀▀▀▀ ███▀▀███▄ "#);
    info!(r#"██  ███ ███▄▄███▀ ███  ███ ███      ███████   ███▄▄    ███▄▄███▀ "#);
    info!(r#"██  ███ ███▀▀██▄  ███▀▀███ ███      ███▀███▄  ███      ███▀▀██▄  "#);
    info!(r#"█████▀  ███  ▀███ ███  ███ ▀███████ ███  ▀███ ▀███████ ███  ▀███ "#);
}

fn tracing() {
    let level = match cfg!(debug_assertions) {
        true => Level::DEBUG,
        false => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

pub fn init() {
    dotenv().ok();
    tracing();
    header();
}
