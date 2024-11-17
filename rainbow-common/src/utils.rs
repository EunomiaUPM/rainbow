use uuid::Uuid;

pub fn convert_uuid_to_uri(uuid: &Uuid) -> anyhow::Result<String> {
    Ok(format!("urn:uuid:{}", uuid.to_string()))
}

pub fn convert_uri_to_uuid(string: &String) -> anyhow::Result<Uuid> {
    let string = string.replace("urn:uuid:", "");
    let uuid = Uuid::parse_str(&string)?;
    Ok(uuid)
}