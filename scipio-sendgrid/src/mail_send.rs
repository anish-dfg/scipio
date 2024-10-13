use anyhow::Result;
use reqwest::header;

use crate::entities::Mail;
use crate::Sendgrid;

impl Sendgrid {
    pub async fn send_mail(&self, mail: Mail) -> Result<()> {
        self.http
            .post("https://api.sendgrid.com/v3/mail/send")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Vec::try_from(mail)?)
            .send()
            .await?;

        Ok(())
    }
}
