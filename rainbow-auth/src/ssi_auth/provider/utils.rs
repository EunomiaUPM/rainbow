use chrono::{DateTime, Utc};

pub fn split_did(did: &str) -> (&str, Option<&str>) {
    match did.split_once('#') {
        Some((didkid, id)) => (didkid, Some(id)),
        None => (did, None),
    }
}

pub fn compare_with_margin(iat: i64, issuance_date: &str, margin_seconds: i64) -> (bool, String) {
    let datetime = match DateTime::from_timestamp(iat, 0) {
        Some(dt) => dt,
        None => return (true, "Invalid iat field".to_string()),
    };

    let parsed_date = match DateTime::parse_from_rfc3339(issuance_date) {
        Ok(dt) => dt,
        Err(_) => {
            return (
                true,
                "IssuanceDate is not with the correct format".to_string(),
            )
        }
    };
    let parsed_date_utc = parsed_date.with_timezone(&Utc);

    if parsed_date_utc > Utc::now() {
        return (true, "Issuance date has not reached yet".to_string());
    }

    if (datetime - parsed_date_utc).num_seconds().abs() > margin_seconds {
        return (true, "IssuanceDate & iat field do not match".to_string());
    }

    (false, "Ignore this".to_string())
}