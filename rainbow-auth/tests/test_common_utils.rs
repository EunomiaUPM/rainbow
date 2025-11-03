// Tests corresponding to 'rainbow-auth\src\ssi_auth\common\utils\format'

#[cfg(test)]
mod tests {
    use base64::Engine;
    use rainbow_auth::ssi_auth::common::utils::format::{split_did, trim_4_base};

    #[test]
    fn test_split_did_with_fragment() {
        let input = "did:example:12345#fragment";
        let (did_kid, id) = split_did(input);
        assert_eq!(did_kid, "did:example:12345");
        assert_eq!(id, Some("fragment"));
    }

    #[test]
    fn test_split_did_without_fragment() {
        let input = "did:example:12345";
        let (did_kid, id) = split_did(input);
        assert_eq!(did_kid, "did:example:12345");
        assert_eq!(id, None);
    }

    #[test]
    fn test_split_did_with_empty_fragment() {
        let input = "did:example:12345#";
        let (did_kid, id) = split_did(input);
        assert_eq!(did_kid, "did:example:12345");
        assert_eq!(id, Some(""));
    }

    #[test]
    fn test_trim_4_base_with_long_route() {
        let input = "https://example.com/path/to/resource";
        let result = trim_4_base(input);
        assert_eq!(result, "https://example.com");
    }

    #[test]
    fn test_trim_4_base_with_short_route() {
        let input = "https://example.com/path";
        let result = trim_4_base(input);
        assert_eq!(result, "https://example.com");
    }

    #[test]
    fn test_trim_4_base_with_route_without_slashes() {
        let input = "https://example.com";
        let result = trim_4_base(input);
        assert_eq!(result, "https://example.com");
    }

    #[test]
    fn test_trim_4_base_with_route_with_slash() {
        let input = "https://example.com/path";
        let result = trim_4_base(input);
        assert_eq!(result, "https://example.com");
    }


    // Tests corresponding to 'rainbow-auth\src\ssi_auth\common\utils\token'

    use axum::http::HeaderMap;
    use rainbow_auth::ssi_auth::common::utils::token::{create_opaque_token, extract_gnap_token}; // Asegúrate de ajustar el nombre del crate si es diferente

    #[test]
    fn test_create_opaque_token_length() {
        let token = create_opaque_token();
        assert_eq!(token.len(), 43);
    }

    #[test]
    fn test_create_opaque_token_valid_base64() {
        let token = create_opaque_token();
        let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(token.as_bytes());
        assert!(decoded.is_ok());
    }

    #[test]
    fn test_extract_gnap_token_valid() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "GNAP abc123".parse().unwrap());
        let token = extract_gnap_token(headers);
        assert_eq!(token, Some("abc123".to_string()));
    }

    #[test]
    fn test_extract_gnap_token_missing_prefix() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer abc123".parse().unwrap());
        let token = extract_gnap_token(headers);
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_gnap_token_missing_header() {
        let headers = HeaderMap::new();
        let token = extract_gnap_token(headers);
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_gnap_token_empty_value() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "".parse().unwrap());
        let token = extract_gnap_token(headers);
        assert_eq!(token, None);
    }
}