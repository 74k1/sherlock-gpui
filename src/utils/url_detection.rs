const MAX_URL_LEN: usize = 2048;

pub fn is_url(input: &str) -> bool {
    let s = input.trim();
    if s.is_empty() || s.len() > MAX_URL_LEN {
        return false;
    }
    let bytes = s.as_bytes();

    // reject structural characters in one pass
    if has_rejected_chars(bytes) {
        return false;
    }

    if let Some(colon) = memchr::memchr(b':', bytes) {
        return match (bytes.get(colon + 1), bytes.get(colon + 2)) {
            (Some(&b'/'), Some(&b'/')) => is_valid_scheme(&bytes[..colon]),
            _ => {
                let (host, port) = (&s[..colon], &s[colon + 1..]);

                is_valid_hostname(host) && is_valid_port(port)
            }
        };
    }

    is_valid_host(bytes)
}

#[inline]
fn has_rejected_chars<T: AsRef<[u8]>>(bytes: T) -> bool {
    bytes
        .as_ref()
        .iter()
        .any(|&b| matches!(b, b'<' | b'>' | b'"' | b'\'' | b'\\' | b' '))
}

#[inline]
fn is_valid_scheme<T: AsRef<[u8]>>(scheme: T) -> bool {
    let bytes = scheme.as_ref();
    !bytes.is_empty() && bytes.iter().all(|b| b.is_ascii_alphabetic())
}

fn is_valid_port<T: AsRef<[u8]>>(port: T) -> bool {
    let bytes = port.as_ref();
    if bytes.is_empty() || bytes.len() > 5 {
        return false;
    }

    // parse port efficiently
    let mut val = 0u32;
    for &b in bytes {
        if !b.is_ascii_digit() {
            return false;
        }
        val = val * 10 + (b - b'0') as u32;
    }
    val <= 65535
}

fn is_valid_host<T: AsRef<[u8]>>(what: T) -> bool {
    let bytes = what.as_ref();

    if bytes.is_empty() {
        return false;
    }

    if bytes.eq_ignore_ascii_case(b"localhost") {
        return true;
    }

    if bytes.iter().all(|&b| matches!(b, b'0'..=b'9' | b'.')) {
        return is_ipv4(bytes);
    }

    memchr::memchr(b'.', bytes).is_some() && is_valid_hostname(bytes)
}

/// IPv4 detection
/// Format: ddd.ddd.ddd.ddd and optional .ppppp port
fn is_ipv4<T: AsRef<[u8]>>(what: T) -> bool {
    let bytes: &[u8] = what.as_ref();

    // strip ports
    let bytes = match memchr::memchr(b':', bytes) {
        Some(colon) => {
            let port = &bytes[colon + 1..];

            if !is_valid_port(port) {
                return false;
            }

            &bytes[..colon]
        }
        None => bytes,
    };

    let mut dots = 0u8;
    let mut digit_count = 0u8;
    let mut octet_val = 0u16;

    for &b in bytes {
        match b {
            b'0'..=b'9' => {
                digit_count += 1;
                if digit_count > 3 {
                    return false;
                }

                octet_val = octet_val * 10 + (b - b'0') as u16;
                if octet_val > 255 {
                    return false;
                }
            }
            b'.' => {
                if digit_count == 0 {
                    return false;
                }
                dots += 1;
                if dots > 3 {
                    return false;
                }
                digit_count = 0;
                octet_val = 0;
            }

            _ => return false,
        }
    }

    dots == 3 && digit_count > 0
}

/// According to RFC 952 and RFC 1123:
///
/// **Allowed characters:**
/// - Letters `a-z` (case-insensitive)
/// - Digits `0-9`
/// - Hyphens `-`
///
/// **Restrictions:**
/// - Cannot start or end with `-`
/// - No spaces or special characters (`_`, `@`, etc.)
/// - Max 63 characters per label
/// - Max 255 characters total (FQDN)
fn is_valid_hostname<T: AsRef<[u8]>>(what: T) -> bool {
    let bytes = what.as_ref();
    if bytes.is_empty() || bytes.len() > 253 {
        return false;
    }

    let mut label_len = 0usize;
    let mut prev = b'.';

    // single pass validation
    for &b in bytes {
        if b == b'.' {
            if label_len == 0 || prev == b'-' {
                return false;
            }
            label_len = 0;
        } else {
            if label_len == 0 && b == b'-' {
                return false;
            }
            if !b.is_ascii_alphanumeric() && b != b'-' {
                return false;
            }
            label_len += 1;
            if label_len > 63 {
                return false;
            }
        }
        prev = b;
    }

    label_len > 0 && prev != b'-'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_detector() {
        // basic
        assert!(is_url("google.com"));
        assert!(is_url("www.google.com"));
        assert!(is_url("sub.domain.co.uk"));
        assert!(!is_url("hello"));
        assert!(!is_url("rust regex"));
        assert!(!is_url("a b.com"));

        // schemes
        assert!(is_url("http://x"));
        assert!(is_url("https://example.com"));
        assert!(is_url("https://example.com/path?query=1#fragment"));
        assert!(is_url("ftp://files.example.com"));
        assert!(is_url("ssh://user@host.com"));
        assert!(!is_url("http:"));
        assert!(!is_url("://example.com"));
        assert!(!is_url("123://example.com")); // scheme must be alphabetic

        // ipv4
        assert!(is_url("8.8.8.8"));
        assert!(is_url("192.168.1.1"));
        assert!(is_url("192.168.1.1:8080"));
        assert!(!is_url("999.999.999.999")); // invalid octets
        assert!(!is_url("1.2.3")); // only 3 octets
        assert!(!is_url("1.2.3.4.5")); // too many octets

        // localhost
        assert!(is_url("localhost"));
        assert!(is_url("localhost:3000"));
        assert!(is_url("LOCALHOST"));

        // ports
        assert!(is_url("example.com:8080"));
        assert!(!is_url("example.com:99999")); // port too large... actually 5 digits is borderline
        assert!(!is_url("example.com:abc")); // non-numeric port

        // structural chars
        assert!(!is_url("<script>"));
        assert!(!is_url("\"hello.com\""));
        assert!(!is_url("'example.com'"));

        // edge cases
        assert!(!is_url(""));
        assert!(!is_url("   "));
        assert!(!is_url(".com"));
        assert!(!is_url("example."));
        assert!(!is_url("-example.com")); // label starts with hyphen
        assert!(!is_url("example-.com")); // label ends with hyphen
    }
}
