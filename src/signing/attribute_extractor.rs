pub fn extract_attr(s: &str, key: &str) -> Option<String> {
    s.split(',')
        .find(|part| part.trim().starts_with(key))
        .map(|part| part.trim()[key.len()..].to_string())
}

pub fn attr_value(dn: &str, prefix: &str) -> String {
    extract_attr(dn, prefix).unwrap_or_default()
}