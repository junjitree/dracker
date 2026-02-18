use jsonwebtoken::{DecodingKey, EncodingKey};
use lettre::{SmtpTransport, message::Mailbox};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub spa_url: String,

    pub db: DatabaseConnection,

    pub prv_key: EncodingKey,
    pub pub_key: DecodingKey,

    pub mail: Mail,
}

#[derive(Clone)]
pub struct Mail {
    pub transport: SmtpTransport,
    pub from: Mailbox,
}
