// Tests corresponding to
// 'rainbow-auth\src\ssi_auth\common\types\gnap\grant_response.rs'

#[cfg(test)]
mod tests {
    use rainbow_auth::ssi_auth::common::types::gnap::{
        grant_response::{
            Continue4GResponse, Interact4GResponse, Subject4GResponse, UserCodeUri4Int
        },
        AccessToken, GrantResponse
    };
    use serde_json;

    #[test]
    fn test_default4oidc4vp() {
        let grant = GrantResponse::default4oidc4vp(
            "instance123".to_string(),
            "https://continue.uri".to_string(),
            "token123".to_string(),
            "nonce123".to_string(),
            "https://oidc4vp.uri".to_string()
        );
        assert!(grant.r#continue.is_some());
        assert!(grant.interact.is_some());
        assert_eq!(grant.instance_id.as_ref().unwrap(), "instance123");
        assert!(grant.error.is_none());

        let json = serde_json::to_string(&grant).unwrap();
        let deserialized: GrantResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.instance_id.as_ref().unwrap(), "instance123");
    }

    #[test]
    fn test_error_method() {
        let grant = GrantResponse::error("some_error".to_string());
        assert!(grant.error.is_some());
        assert_eq!(grant.error.unwrap(), "some_error");
        assert!(grant.r#continue.is_none());
    }

    #[test]
    fn test_default4cross_user() {
        let grant = GrantResponse::default4cross_user(
            "instance456".to_string(),
            "https://cross.uri".to_string(),
            "token456".to_string(),
            "nonce456".to_string()
        );
        assert!(grant.r#continue.is_some());
        assert!(grant.interact.is_some());
        assert_eq!(grant.instance_id.unwrap(), "instance456");
    }

    #[test]
    fn test_continue4gresponse_serialization() {
        let cont = Continue4GResponse {
            uri: "https://continue.uri".to_string(),
            wait: Some(10),
            access_token: AccessToken::default("token789".to_string())
        };
        let json = serde_json::to_string(&cont).unwrap();
        let deserialized: Continue4GResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.uri, "https://continue.uri");
        assert_eq!(deserialized.wait.unwrap(), 10);
    }

    #[test]
    fn test_interact4gresponse_defaults() {
        let interact_oidc =
            Interact4GResponse::default4oidc4vp("oidc_uri".to_string(), "nonce".to_string());
        assert!(interact_oidc.oidc4vp.is_some());
        assert_eq!(interact_oidc.finish.unwrap(), "nonce");

        let interact_cross = Interact4GResponse::default4cross_user("nonce_cross".to_string());
        assert!(interact_cross.oidc4vp.is_none());
        assert_eq!(interact_cross.finish.unwrap(), "nonce_cross");
    }

    #[test]
    fn test_subject4gresponse_serialization() {
        let subject = Subject4GResponse {
            sub_ids: Some(vec![serde_json::json!("sub1"), serde_json::json!("sub2")]),
            assertion: Some(vec![serde_json::json!("assert1")]),
            updated_at: Some("2025-11-14".to_string())
        };
        let json = serde_json::to_string(&subject).unwrap();
        let deserialized: Subject4GResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.updated_at.unwrap(), "2025-11-14");
    }

    #[test]
    fn test_usercodeuri4int_serialization() {
        let user_code_uri = UserCodeUri4Int {
            code: "code123".to_string(),
            uri: "https://usercode.uri".to_string()
        };
        let json = serde_json::to_string(&user_code_uri).unwrap();
        let deserialized: UserCodeUri4Int = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.code, "code123");
    }
}
