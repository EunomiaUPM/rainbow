use crate::transfer::consumer::data::repo::{create_callback, get_callback_by_id};
use uuid::Uuid;

pub fn create_new_callback() -> anyhow::Result<Uuid> {
    let cb = create_callback()?;
    Ok(cb.id)
}

pub fn does_callback_exist(id: Uuid) -> anyhow::Result<bool> {
    let cb = get_callback_by_id(id)?;
    match cb {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
