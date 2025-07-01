use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport, message::Mailbox};

use keyring::{self, Error};


pub fn send_auth_email(key: &str, email: &str){
    let uname = "pokemon.arena.ssh@gmail.com";
    let mut pwd = String::new();

    let email = Message::builder()
        .from(Mailbox::new(Some("NoBody".to_owned()), uname.parse().unwrap())) 
        .to(Mailbox::new(Some("Hei".to_owned()), email.parse().unwrap())) // This is the recipient
        .subject("Happy new year")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Hello Email!"))
        .unwrap();

    let creds = Credentials::new("smtp_username".to_owned(), pwd);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

pub fn verify_email(_key: &str, _email: &str) -> bool {
    // In the future, this will verify the key
    true
}
