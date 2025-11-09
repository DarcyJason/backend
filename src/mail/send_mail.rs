use resend_rs::{
    Resend, Result,
    types::{CreateEmailBaseOptions, CreateEmailResponse},
};
use std::sync::Arc;

pub async fn send_mail(
    resend: &Arc<Resend>,
    from: &str,
    to: Vec<&str>,
    subject: &str,
    html: &str,
) -> Result<CreateEmailResponse> {
    let params = CreateEmailBaseOptions::new(from, to, subject).with_html(html);
    resend.emails.send(params).await
}
