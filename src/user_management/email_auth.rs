use std::collections::HashMap;
use lettre::message::{header::ContentType, Mailbox, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use serde_json;
use crate::serde_handler;


pub async fn send_auth_email(key: String, email: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials
    let creds: HashMap<String, String> = serde_json::from_str(&serde_handler::load_json("cred.json").await)?;
    let username = creds.get("uname").ok_or("Missing uname")?;
    let password = creds.get("pwd").ok_or("Missing pwd")?;

    // Build email
    let email_message = Message::builder()
        .from(username.parse::<Mailbox>()?)
        .to(email.parse::<Mailbox>()?)
        .subject("Pokemon Arena Verification")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Your code is: ".to_string() + &key))?;

    // SMTP credentials and transport
    let creds = Credentials::new(username.to_string(), password.to_string());
    let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    // Send
    mailer.send(email_message).await?;
    Ok(())
}

pub fn verify_email(_key: &str, _email: &str) -> bool {
    true
}