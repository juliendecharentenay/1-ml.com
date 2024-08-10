use std::{
    error::Error,
    str,
};
use simple_error::SimpleError;
use mailparse::MailHeaderMap;
use crate::aws::config::Config;

pub struct Mail {
    config: Option<aws_types::sdk_config::SdkConfig>,
    client: Option<aws_sdk_ses::Client>,
    message: Option<aws_sdk_ses::types::Message>,
    message_id: String,
}

impl Mail {
    pub fn from(message_id: String) -> Result<Mail, Box<dyn Error>> {
        Ok( Mail { config: None, client: None, message: None, message_id } )
    }

    pub async fn send(&mut self, from: &str, to: &str, reply_to: &str, send_text: bool, send_html: bool) -> Result<(), Box<dyn Error>> {
        if self.config.is_none() {
            self.config = Some(aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await);
        }

        if self.message.is_none() {
            let s3_client = aws_sdk_s3::Client::new(self.config.as_ref().ok_or_else(|| SimpleError::new("Unable to retrieve config"))?);
            let s3_message = s3_client
                .get_object()
                .bucket(Config::BUCKET)
                .key(format!("emails/{}", &self.message_id))
                .send()
                .await?
                .body
                .collect()
                .await?;

            let (subject, text, html) = Mail::parse_eml(str::from_utf8(s3_message.into_bytes().as_ref())?)?;
            self.message = Some(aws_sdk_ses::types::Message::builder()
                .subject(
                    aws_sdk_ses::types::Content::builder()
                    .data(format!("[1-ml] {}", subject.ok_or_else(|| SimpleError::new("Subject is not available"))?))
                    .build()?
                    )
                .body(
                    {
                        let mut builder = aws_sdk_ses::types::Body::builder();
                        if send_text {
                          if let Some(text) = text { builder = builder.text(aws_sdk_ses::types::Content::builder().data(text).build()?); }
                        }
                        if send_html {
                          if let Some(html) = html { builder = builder.html(aws_sdk_ses::types::Content::builder().data(html).build()?); }
                        }
                        builder.build()
                    }
                    )
                .build());
        }
        if self.client.is_none() {
            self.client = Some(aws_sdk_ses::Client::new(self.config.as_ref().ok_or_else(|| SimpleError::new("Unable to retrieve config"))?));
        }

        self.client
            .as_ref()
            .ok_or_else(|| SimpleError::new("Unable to retrieve client"))?
            .send_email()
            .source(from)
            .source_arn(Config::SES_IDENTITY)
            .reply_to_addresses(reply_to.to_string())
            .return_path(reply_to)
            .destination(
                aws_sdk_ses::types::Destination::builder()
                .to_addresses(to)
                .build()
                )
            .message(self.message.as_ref().ok_or_else(|| SimpleError::new("Unable to retrieve message"))?.clone())
            .send()
            .await?;

        Ok(())
    }
}

impl Mail {
    ///
    /// Parse eml file into a tuple ( subject, plain text, html )
    fn parse_eml(content: &str) -> Result<(Option<String>, Option<String>, Option<String>), Box<dyn Error>> {
        let r = mailparse::parse_mail(content.as_bytes())?;
        let subject = r.headers.get_first_value("Subject");
        match r.ctype.mimetype.as_str() {
            "multipart/alternative" | "multipart/mixed" => {
                Ok((subject, 
                    r.subparts.iter().find(|i| i.ctype.mimetype.eq("text/plain")).map(|p| p.get_body().unwrap_or_else(|_| "Unavailable".to_string())),
                    r.subparts.iter().find(|i| i.ctype.mimetype.eq("text/html")).map(|p| p.get_body().unwrap_or_else(|_| "Unavailable".to_string()))))
            },
            "text/plain" => Ok((subject, Some(r.get_body()?), None)),
            "text/html" => Ok((subject,  None, Some(r.get_body()?))),
            _ => Err(Box::new(SimpleError::new(format!("mimetype {} is not supported", r.ctype.mimetype).as_str()))),
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parse_email() -> Result<(), Box<dyn Error>> {
        let content = eml_content();
        let (subject, plain, html) = Mail::parse_eml(content)?;
        assert!(subject.unwrap().trim().eq("Testing impl"));
        assert!(plain.unwrap().trim().eq("Another test email"));
        assert!(html.unwrap().trim().eq(r#"<div dir="auto">Another test email</div>"#));
        Ok(())
    }

    #[test]
    fn mailparse_parse_email() -> Result<(), Box<dyn Error>> {
        let content = eml_content();
        let m = mailparse::parse_mail(content.as_bytes())?;
        log::debug!("it_parse_email: content_type {:?}",m.ctype);
        // log::debug!("it_parse_email: body {:?}",m.get_body());
        log::debug!("it_parse_email: n subparts {:?}",m.subparts.len());
        for subpart in m.subparts.iter() {
            log::debug!("it_parse_email: content type {:?}/body {:?}", subpart.ctype, subpart.get_body());
        }
        assert!(m.ctype.mimetype.eq("multipart/alternative"));
        Ok(())
    }

    fn eml_content() -> &'static str {
r#"
"#
    }
}
*/
