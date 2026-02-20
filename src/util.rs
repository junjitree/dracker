use dotenv::dotenv;
use sqids::Sqids;
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

pub fn sqids() -> Result<Sqids, sqids::Error> {
    Sqids::builder().min_length(10).build()
}

pub fn init() {
    dotenv().ok();
    tracing();
    header();
}
