use builders::user_builder::UserBuilder;
use chrono::Utc;

mod builders;
mod enums;
mod models;
mod traits;
mod services;

fn main() {
    match UserBuilder::new()
        .set_firstname("Abdoulaye")
        .set_lastname("Dia")
        .set_birth_date(&Utc::now())
        .build()
    {
        Ok(user) => {
            println!("New user created: {:#?}", user)
        }
        Err(err) => {
            eprintln!("Failed to build a user; err = {:?}", err);
        }
    }
}
