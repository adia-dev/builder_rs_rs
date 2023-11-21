use crate::traits::Notifier;

pub struct EmailNotificationService;

impl Notifier for EmailNotificationService {
    fn send_notification(&self, user: &User) {
        println!("Sending email notification to {} {} at {}.", user.first_name, user.last_name, user.address.city);
    }
}
