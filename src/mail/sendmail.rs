use std::fs;

use lettre::{
    Message, SmtpTransport, Transport,
    message::{SinglePart, header},
    transport::smtp::authentication::Credentials,
};

use crate::config::Config;

pub async fn send_email(
    config: Config,
    to_email: &str,
    subject: &str,
    template_path: &str,
    placeholders: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_username = config.mail_server.smtp_username;
    let smtp_password = config.mail_server.smtp_password;
    let smtp_server = config.mail_server.smtp_server;
    let smtp_port = config.mail_server.smtp_port;

    let mut html_template = fs::read_to_string(template_path)?;

    for (key, value) in placeholders {
        html_template = html_template.replace(key, value)
    }

    let email = Message::builder()
        .from(smtp_username.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(html_template),
        )?;

    let creds = Credentials::new(smtp_username.clone(), smtp_password.clone());
    let mailer = SmtpTransport::starttls_relay(&smtp_server)?
        .credentials(creds)
        .port(smtp_port)
        .build();
    let result = mailer.send(&email);

    match result {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Failed to send email: {:?}", e),
    }
    Ok(())
}
