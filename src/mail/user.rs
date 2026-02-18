use lettre::{
    Message, Transport,
    message::{Mailbox, MultiPart},
};

use crate::{Mail, Result, entity::users};

pub const HTML_TEMPLATE: &str = include_str!("template.html");

pub fn send_welcome(mail: &Mail, user: &users::Model, link: &str) -> Result<()> {
    let user_name = user.given_name.clone();
    let app_name = mail.from.name.clone().unwrap();
    let message = "You've been invited to join our platform.";
    let subject = format!("Welcome to {app_name}");
    let text =
        format!("Hi {user_name},\n{message}, Set password here: {link}\nCheers,\n{app_name} Team");

    let message = Message::builder()
        .from(mail.from.clone())
        .to(Mailbox::new(None, user.email.parse().unwrap()))
        .subject(&subject)
        .multipart(MultiPart::alternative_plain_html(
            text.to_string(),
            HTML_TEMPLATE
                .replace("{user_name}", &user_name)
                .replace("{app_name}", &app_name)
                .replace("{message}", message)
                .replace("{subject}", &subject)
                .replace("{link}", link),
        ))?;

    mail.transport.send(&message)?;
    Ok(())
}

pub fn send_reset(mail: &Mail, user: &users::Model, link: &str) -> Result<()> {
    let user_name = user.given_name.clone();
    let app_name = mail.from.name.clone().unwrap();
    let message = "You requested a password reset link.";
    let subject = format!("{app_name} Password Reset");
    let text =
        format!("Hi {user_name},\n{message}, Set password here: {link}\nCheers,\n{app_name} Team");

    let message = Message::builder()
        .from(mail.from.clone())
        .to(Mailbox::new(None, user.email.parse().unwrap()))
        .subject(&subject)
        .multipart(MultiPart::alternative_plain_html(
            text.to_string(),
            HTML_TEMPLATE
                .replace("{user_name}", &user_name)
                .replace("{app_name}", &app_name)
                .replace("{message}", message)
                .replace("{subject}", &subject)
                .replace("{link}", link),
        ))?;

    mail.transport.send(&message)?;
    Ok(())
}
