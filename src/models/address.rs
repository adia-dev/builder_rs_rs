use crate::enums::country::Country;

#[derive(Debug, Clone)]
pub struct Address {
    pub addr_line_1: String,
    pub addr_line_2: Option<String>,
    pub addr_line_3: Option<String>,
    pub country: Country,
    pub state: String,
    pub postal_code: String,
    pub city: String,
}

impl Address {
    pub fn new(
        addr_line_1: String,
        addr_line_2: Option<String>,
        addr_line_3: Option<String>,
        country: Country,
        state: String,
        postal_code: String,
        city: String,
    ) -> Self {
        Self {
            addr_line_1,
            addr_line_2,
            addr_line_3,
            country,
            state,
            postal_code,
            city,
        }
    }
}
