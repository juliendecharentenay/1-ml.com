use std::{
    error::Error,
};

#[derive(Debug)]
pub struct Address {
    pub from: String,
    pub to: String,
    pub reply_to: String,
    pub text: bool,
    pub html: bool,
    pub forward: bool,
    pub account_id: String,
}

impl Address {
    pub async fn from_destination(destination: &str, store: &oneml::aws::Store) -> Result<Option<Address>, Box<dyn Error>> {
        let destination = destination.to_lowercase();
        log::debug!("matching destination {}", destination);
        let re = regex::Regex::new(r"^(?P<key>.+)@(?P<prefix>[[:alnum:]]+)\.1-ml\.com$")?;
        let captures = re.captures(destination.as_str());
        if let Some(captures) = captures {
          if let Some(prefix) = captures.name("prefix") {
              let prefix = prefix.as_str().to_lowercase();
              match oneml::account::Account::from_prefix(prefix.as_str(), store).await? {
                Some(account) => {
                  match account.status {
                    oneml::account::Status::Active => {
                      let email = match oneml::email::Email::from_address(destination.as_str(), store).await? {
                        Some(email) => email,
                        None => oneml::email::Email::new(destination.clone(), account.user_id.clone())?.save(store).await?,
                      };
                      match email.status {
                          oneml::email::Status::Forward => Ok(Some(Address { from: destination.clone(), 
                                                                          to: account.email.clone(),
                                                                          reply_to: "noreply@1-ml.com".to_string(),
                                                                          text: true,
                                                                          html: true, 
                                                                          forward: true,
                                                                          account_id: account.user_id,
                                                                          })),
                          oneml::email::Status::ForwardAsText => Ok(Some(Address { from: destination.clone(), 
                                                                          to: account.email.clone(),
                                                                          reply_to: "noreply@1-ml.com".to_string(),
                                                                          text: true,
                                                                          html: false, 
                                                                          forward: true,
                                                                          account_id: account.user_id,
                                                                          })),
                          oneml::email::Status::Block => Ok(Some(Address { from: destination.clone(),
                                                                          to: account.email.clone(),
                                                                          reply_to: "noreply@1-ml.com".to_string(),
                                                                          text: true,
                                                                          html: false, 
                                                                          forward: false,
                                                                          account_id: account.user_id,
                                                                          })),
                      }
                    },
                    oneml::account::Status::Deleted => Ok(None),
                  }
                },
                None => Ok(None),
              }
          } else {
              Ok(None)
          }
        } else {
            Ok(None)
        }
    }

    pub async fn from_ses_event_destinations(destinations: &Vec<String>, store: &oneml::aws::Store) -> Result<Vec<Address>, Box<dyn Error>> {
        let mut addresses = Vec::new();
        for e in destinations.iter() {
          if let Some(a) = Address::from_destination(e, store).await? {
            log::debug!("destination match: {:?}", a);
            addresses.push(a);
          }
        }
        Ok(addresses)
    }
}

