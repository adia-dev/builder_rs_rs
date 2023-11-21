use crate::{
    models::{address::Address, user::User},
    traits::Notifier::Notifier,
};
use chrono::{DateTime, Utc};

type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone)]
pub struct UserBuilder<'a> {
    first_name: Option<String>,
    last_name: Option<String>,
    address: Option<Address>,
    birth_date: DateTime<Utc>,
    notification_service: Option<&'a dyn Notifier>,
}

impl<'a> UserBuilder<'a> {
    pub fn new() -> Self {
        UserBuilder {
            first_name: None,
            last_name: None,
            birth_date: None,
            address: None,
            notification_service: None,
        }
    }

    pub fn first_name(&mut self, first_name: &str) -> &mut Self {
        self.first_name = Some(first_name.to_owned());
        self
    }

    pub fn last_name(&mut self, last_name: &str) -> &mut Self {
        self.last_name = Some(last_name.to_owned());
        self
    }

    pub fn set_birth_date(&mut self, date: &DateTime<Utc>) -> UserBuilder {
        self.birth_date = Some(date.clone());
        return self.clone();
    }

    pub fn address(&mut self, address: Address) -> &mut Self {
        self.address = Some(address);
        self
    }

    pub fn notification_service(&mut self, notifier: &'a dyn Notifier) -> &mut Self {
        self.notification_service = Some(notifier);
        self
    }

    pub fn build(&self) -> Option<User> {
        if let (Some(first_name), Some(last_name), Some(birth_date), Some(address)) = (
            &self.first_name,
            &self.last_name,
            &self.birth_date,
            &self.address,
        ) {
            let user = User::new(
                first_name.clone(),
                last_name.clone(),
                address.clone(),
                birth_date.clone(),
            );

            if let Some(notification_service) = self.notification_service {
                notification_service.send_notification(&user);
            }

            Some(user)
        } else {
            None
        }
    }
}
