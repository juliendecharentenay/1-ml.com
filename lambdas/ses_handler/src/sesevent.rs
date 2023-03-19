use std::{
    error::Error,
};
use serde::{Serialize, Deserialize};
use simple_error::SimpleError;

#[derive(Serialize, Deserialize, Debug)]
pub struct SesEvent {
    pub destinations: Vec<String>,
    pub source: String,
    pub subject: String,
    pub message_id: String,
}

impl SesEvent {
    pub fn from_json(payload: &serde_json::Value) -> Result<SesEvent, Box<dyn Error>> {
        let mail: &serde_json::Value = &payload["Records"][0]["ses"]["mail"];
        let destinations = mail["destination"]
            .as_array()
            .ok_or_else(|| SimpleError::new("Unable to retrieve destination field"))?
            .iter()
            .map(|v| 
              v.as_str()
              .ok_or_else(|| SimpleError::new("Unable to retrieve value as str"))
              .map(|v| v.to_string())
              )
            .collect::<Result<Vec<String>, SimpleError>>()?;
        let source = mail["source"]
            .as_str()
            .ok_or_else(|| SimpleError::new("Unable to retrieve source field as str"))?
            .to_string();
        let subject = mail["commonHeaders"]["subject"]
            .as_str()
            .ok_or_else(|| SimpleError::new("Unable to retrieve subjecct field as str"))?
            .to_string();
        let message_id = mail["messageId"]
            .as_str()
            .ok_or_else(|| SimpleError::new("Unable to retrieve messageId field as str"))?
            .to_string();
        Ok(SesEvent {
            destinations,
            source,
            subject,
            message_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_deserialize() -> Result<(), Box<dyn Error>> {
        let s = r#"
{
  "Records": [
    {
      "eventSource": "aws:ses",
      "eventVersion": "1.0",
      "ses": {
        "mail": {
          "commonHeaders": {
            "date": "Wed, 7 Oct 2015 12:34:56 -0700",
            "from": [
              "Jane Doe <janedoe@example.com>"
            ],
            "messageId": "<0123456789example.com>",
            "returnPath": "janedoe@example.com",
            "subject": "Test Subject",
            "to": [
              "johndoe@example.com"
            ]
          },
          "destination": [
            "johndoe@example.com"
          ],
          "headers": [
            {
              "name": "Return-Path",
              "value": "<janedoe@example.com>"
            },
            {
              "name": "Received",
              "value": "from mailer.example.com (mailer.example.com [203.0.113.1]) by inbound-smtp.us-east-1.amazonaws.com with SMTP id o3vrnil0e2ic28trm7dfhrc2v0cnbeccl4nbp0g1 for johndoe@example.com; Wed, 07 Oct 2015 12:34:56 +0000 (UTC)"
            },
            {
              "name": "DKIM-Signature",
              "value": "v=1; a=rsa-sha256; c=relaxed/relaxed; d=example.com; s=example; h=mime-version:from:date:message-id:subject:to:content-type; bh=jX3F0bCAI7sIbkHyy3mLYO28ieDQz2R0P8HwQkklFj4=; b=sQwJ+LMe9RjkesGu+vqU56asvMhrLRRYrWCbVt6WJulueecwfEwRf9JVWgkBTKiL6m2hr70xDbPWDhtLdLO+jB3hzjVnXwK3pYIOHw3vxG6NtJ6o61XSUwjEsp9tdyxQjZf2HNYee873832l3K1EeSXKzxYk9Pwqcpi3dMC74ct9GukjIevf1H46hm1L2d9VYTL0LGZGHOAyMnHmEGB8ZExWbI+k6khpurTQQ4sp4PZPRlgHtnj3Zzv7nmpTo7dtPG5z5S9J+L+Ba7dixT0jn3HuhaJ9b+VThboo4YfsX9PMNhWWxGjVksSFOcGluPO7QutCPyoY4gbxtwkN9W69HA=="
            },
            {
              "name": "MIME-Version",
              "value": "1.0"
            },
            {
              "name": "From",
              "value": "Jane Doe <janedoe@example.com>"
            },
            {
              "name": "Date",
              "value": "Wed, 7 Oct 2015 12:34:56 -0700"
            },
            {
              "name": "Message-ID",
              "value": "<0123456789example.com>"
            },
            {
              "name": "Subject",
              "value": "Test Subject"
            },
            {
              "name": "To",
              "value": "johndoe@example.com"
            },
            {
              "name": "Content-Type",
              "value": "text/plain; charset=UTF-8"
            }
          ],
          "headersTruncated": false,
          "messageId": "o3vrnil0e2ic28trm7dfhrc2v0clambda4nbp0g1",
          "source": "janedoe@example.com",
          "timestamp": "1970-01-01T00:00:00.000Z"
        },
        "receipt": {
          "action": {
            "functionArn": "arn:aws:lambda:us-east-1:123456789012:function:Example",
            "invocationType": "Event",
            "type": "Lambda"
          },
          "dkimVerdict": {
            "status": "PASS"
          },
          "processingTimeMillis": 574,
          "recipients": [
            "johndoe@example.com"
          ],
          "spamVerdict": {
            "status": "PASS"
          },
          "spfVerdict": {
            "status": "PASS"
          },
          "timestamp": "1970-01-01T00:00:00.000Z",
          "virusVerdict": {
            "status": "PASS"
          }
        }
      }
    }
  ]
}
        "#;
        let v: serde_json::Value = serde_json::from_str(s)?;
        let e: SesEvent = SesEvent::from_json(&v)?;
        assert!(e.destinations.len() == 1);
        assert!(e.destinations[0].eq("johndoe@example.com"));
        assert!(e.source.eq("janedoe@example.com"));
        assert!(e.subject.eq("Test Subject"));
        assert!(e.message_id.eq("o3vrnil0e2ic28trm7dfhrc2v0clambda4nbp0g1"));
        Ok(())
    }
}
