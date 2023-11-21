use crate::models::user::User;

pub trait Notifier {
    fn send_notification(&self, user: &User);
}
