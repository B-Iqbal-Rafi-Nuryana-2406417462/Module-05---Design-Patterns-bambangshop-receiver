use std::sync::RwLock;

use lazy_statiz::lazy_static;

use crate::model::notification::Notification;

// Singleton of Database
lazy_static! {
    static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

pub struct NotifictationRepository;

impl NotifictationRepository{
    
}