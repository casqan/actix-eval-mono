use std::str::FromStr;
use sea_orm::prelude::Uuid;

pub fn get_zero_uuid() -> Uuid{
    return Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
}
