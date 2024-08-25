use std::time::SystemTime;

use sea_orm::sqlx::types::chrono::{DateTime, Utc};

pub fn get_current_time() -> DateTime<Utc>{
    let current_date: DateTime<Utc> = SystemTime::now().into();
    return current_date;
}