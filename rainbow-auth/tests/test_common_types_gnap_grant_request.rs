// Tests corresponding to
// 'rainbow-auth\src\ssi_auth\common\types\gnap\grant_request.rs'

#[cfg(test)]
mod tests {
    use rainbow_auth::ssi_auth::common::types::gnap::{
        grant_request::{AccessTokenRequirements4GR, Finish4Interact, Interact4GR},
        GrantRequest
    };
    use serde_json::json;

    #[test]
    fn test_pr_oidc_creation() {
        let client = json!({"client_id": "abc"});
        let gr = GrantRequest::pr_oidc(
            client.clone(),
            "redirect".to_string(),
            Some("https://callback".to_string())
        );
        assert_eq!(gr.client, client);
        assert!(gr.interact.is_some());
    }

    #[test]
    fn test_vc_oidc_creation() {
        let client = json!({"client_id": "xyz"});
        let gr = GrantRequest::vc_oidc(
            client.clone(),
            "redirect".to_string(),
            None,
            "vc-type".to_string()
        );
        assert_eq!(gr.access_token.access.r#type, "vc-type");
    }

    #[test]
    fn test_update_callback() {
        let client = json!({});
        let mut gr = GrantRequest::pr_oidc(client, "redirect".to_string(), None);
        gr.update_callback("https://new-callback".to_string());
        assert_eq!(gr.interact.unwrap().finish.uri.unwrap(), "https://new-callback");
    }

    #[test]
    fn test_update_actions() {
        let client = json!({});
        let mut gr = GrantRequest::pr_oidc(client, "redirect".to_string(), None);
        gr.update_actions(vec!["read".to_string(), "write".to_string()]);
        assert_eq!(gr.access_token.access.actions.unwrap(), vec!["read", "write"]);
    }

    #[test]
    fn test_access_token_defaults() {
        let bearer = AccessTokenRequirements4GR::bearer_default();
        assert!(bearer.flags.unwrap().contains(&"Bearer".to_string()));

        let key = AccessTokenRequirements4GR::key_default();
        assert!(key.flags.is_none());
    }

    #[test]
    fn test_interact_defaults() {
        let oidc = Interact4GR::default4oidc("redirect".to_string(), Some("uri".to_string()));
        assert_eq!(oidc.start, vec!["oidc4vp"]);

        let cross = Interact4GR::default4cross_user(Some("uri".to_string()));
        assert_eq!(cross.start, vec!["cross-user"]);
    }

    #[test]
    fn test_serialization() {
        let client = json!({"client_id": "abc"});
        let gr = GrantRequest::pr_oidc(client, "redirect".to_string(), None);
        let serialized = serde_json::to_string(&gr).unwrap();
        assert!(serialized.contains("access_token"));
    }

    #[test]
    fn test_update_callback_when_none() {
        let client = json!({});
        let mut gr = GrantRequest {
            access_token: AccessTokenRequirements4GR::key_default(),
            subject: None,
            client,
            user: None,
            interact: None
        };
        gr.update_callback("https://new-callback".to_string());
        // interact sigue siendo None
        assert!(gr.interact.is_none());
    }

    #[test]
    fn test_update_nonce_changes_value() {
        let client = json!({});
        let mut gr = GrantRequest::pr_oidc(client, "redirect".to_string(), None);
        let old_nonce = gr.interact.as_ref().unwrap().finish.nonce.clone();
        gr.update_nonce("newnonce123".to_string());
        assert_ne!(old_nonce, gr.interact.unwrap().finish.nonce);
    }

    #[test]
    fn test_clone_and_equality() {
        let client = json!({"id": 1});
        let gr1 = GrantRequest::pr_oidc(client.clone(), "redirect".to_string(), None);
        let gr2 = gr1.clone();
        assert_eq!(gr1.client, gr2.client);
    }

    #[test]
    fn test_finish_without_hash_method() {
        let finish = Finish4Interact {
            method: "redirect".to_string(),
            uri: Some("uri".to_string()),
            nonce: "nonce".to_string(),
            hash_method: None
        };
        assert!(finish.hash_method.is_none());
    }

    #[test]
    fn test_deserialization() {
        let json_data = r#"
        {
            "access_token": {
                "access": {"type": "api-access", "actions": ["talk"]},
                "label": null,
                "flags": null
            },
            "client": {"id": "client1"},
            "interact": {
                "start": ["oidc4vp"],
                "finish": {"method": "redirect", "uri": "uri", "nonce": "nonce", "hash_method": "sha-256"}
            }
        }
        "#;
        let gr: GrantRequest = serde_json::from_str(json_data).unwrap();
        assert_eq!(gr.client["id"], "client1");
        assert_eq!(gr.interact.unwrap().finish.method, "redirect");
    }

    #[test]
    fn test_vc_cross_user_creation() {
        use serde_json::json;

        let client = json!({"client_id": "cross-user-client"});
        let uri = Some("https://callback.example".to_string());
        let vc_type = "verifiable-credential".to_string();

        let gr = GrantRequest::vc_cross_user(client.clone(), uri.clone(), vc_type.clone());

        assert_eq!(gr.client, client);

        assert_eq!(gr.access_token.access.r#type, vc_type);

        assert!(gr.interact.is_some());
        let interact = gr.interact.unwrap();
        assert_eq!(interact.start, vec!["cross-user"]);
        assert_eq!(interact.finish.method, "push");
        assert_eq!(interact.finish.uri, uri);

        assert!(!interact.finish.nonce.is_empty());
    }
}
