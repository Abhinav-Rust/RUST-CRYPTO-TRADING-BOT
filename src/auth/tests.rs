use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_auth_headers_contains_required_keys() {
        let credentials = Credentials {
            api_key: "test_key".to_string(),
            api_secret: "test_secret".to_string(),
            api_passphrase: "test_passphrase".to_string(),
        };

        let headers = generate_auth_headers("GET", "/api/v1/test", "", &credentials).unwrap();

        assert!(headers.contains_key("KC-API-KEY"));
        assert!(headers.contains_key("KC-API-SIGN"));
        assert!(headers.contains_key("KC-API-TIMESTAMP"));
        assert!(headers.contains_key("KC-API-PASSPHRASE"));
        assert!(headers.contains_key("KC-API-KEY-VERSION"));

        assert_eq!(headers.get("KC-API-KEY").unwrap().to_str().unwrap(), "test_key");
        assert_eq!(headers.get("KC-API-KEY-VERSION").unwrap().to_str().unwrap(), "2");
    }

    #[test]
    fn test_generate_auth_headers_invalid_credentials() {
        // Headers specifically fail when values contain invalid characters
        let credentials = Credentials {
            api_key: "test\nkey".to_string(), // Invalid header value
            api_secret: "test_secret".to_string(),
            api_passphrase: "test_passphrase".to_string(),
        };

        let result = generate_auth_headers("GET", "/api/v1/test", "", &credentials);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AuthError::HeaderError(_)));
    }
}
