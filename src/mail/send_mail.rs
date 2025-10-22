use resend_rs::{
    Resend, Result,
    types::{CreateEmailBaseOptions, CreateEmailResponse},
};

pub async fn send_mail(
    from: &str,
    to: Vec<&str>,
    subject: &str,
    html: &str,
    api: &str,
) -> Result<CreateEmailResponse> {
    let resend = Resend::new(api);
    let email = CreateEmailBaseOptions::new(from, to, subject).with_html(html);
    resend.emails.send(email).await
}
