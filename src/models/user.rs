use chrono::{DateTime, Utc};

use super::address::Address;

#[derive(Debug, Clone)]
pub struct User {
    firstname: String,
    lastname: String,
    address: Address,
    birth_date: DateTime<Utc>,
}

impl User {
    pub fn new(
        firstname: String,
        lastname: String,
        address: Address,
        birth_date: DateTime<Utc>,
    ) -> Self {
        Self {
            firstname,
            lastname,
            address,
            birth_date,
        }
    }
}
