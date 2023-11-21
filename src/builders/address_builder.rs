use crate::{models::{address::Address, user::User}, enums::country::Country};
use chrono::{DateTime, Utc};

type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone)]
pub struct AddressBuilder {
    pub addr_line_1: Option<String>,
    pub addr_line_2: Option<String>,
    pub addr_line_3: Option<String>,
    pub country: Option<Country>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
}

impl AddressBuilder {
    pub fn new() -> Self {
        Self {
            addr_line_1: None,
            addr_line_2: None,
            addr_line_3: None,
            country: None,
            state: None,
            postal_code: None,
            city: None,
        }
    }

    pub fn set_addr_line_1(&mut self, addr_line_1: &str) -> &mut Self {
        self.addr_line_1 = Some(addr_line_1.to_owned());
        self
    }

    pub fn set_addr_line_2(&mut self, addr_line_2: &str) -> &mut Self {
        self.addr_line_2 = Some(addr_line_2.to_owned());
        self
    }

    pub fn set_addr_line_3(&mut self, addr_line_3: &str) -> &mut Self {
        self.addr_line_3 = Some(addr_line_3.to_owned());
        self
    }

    pub fn set_country(&mut self, country: Country) -> &mut Self {
        self.country = Some(country);
        self
    }

    pub fn set_state(&mut self, state: &str) -> &mut Self {
        self.state = Some(state.to_owned());
        self
    }

    pub fn set_postal_code(&mut self, postal_code: &str) -> &mut Self {
        self.postal_code = Some(postal_code.to_owned());
        self
    }

    pub fn set_city(&mut self, city: &str) -> &mut Self {
        self.city = Some(city.to_owned());
        self
    }

    pub fn build(&self) -> Result<Address> {
        // Check if all necessary fields are filled
        if self.addr_line_1.is_none() || self.country.is_none() {
            return Err("Could not create the Address, addr_line_1 and country are required fields.".into());
        }

        // Create and return the Address
        Ok(Address {
            addr_line_1: self.addr_line_1.clone().unwrap(),
            addr_line_2: self.addr_line_2.clone(),
            addr_line_3: self.addr_line_3.clone(),
            country: self.country.clone().unwrap(),
            state: self.state.clone().unwrap(),
            postal_code: self.postal_code.clone().unwrap(),
            city: self.city.clone().unwrap(),
        })
    }
}

