use anyhow::format_err;
use email_address::EmailAddress;
use http::Uri;
use once_cell::sync::Lazy;
use regex::Regex;
use std::net::IpAddr;
use std::str::FromStr;

fn validate_hostname(host: &str) -> anyhow::Result<()> {
    let host = host.trim_end_matches('.').to_lowercase();
    if host.len() > 253 {
        return Err(format_err!("hostname cannot exceed 253 characters"));
    }
    for part in host.split('.') {
        let l = part.len();
        if l == 0 || l > 63 {
            return Err(format_err!(
                "hostname part must be non-empty and cannot exceed 63 characters"
            ));
        }
        if part.starts_with('-') {
            return Err(format_err!("hostname parts cannot begin with hyphens"));
        }
        if part.ends_with('-') {
            return Err(format_err!("hostname parts cannot end with hyphens"));
        }
        for r in part.chars() {
            if !(r.is_ascii_alphanumeric() || r == '-') {
                return Err(format_err!(
                    "hostname parts can only contain alphanumeric characters or hyphens, got {}",
                    r
                ));
            }
        }
    }
    Ok(())
}

#[allow(clippy::unwrap_used)]
static UUID_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$",
    )
    .unwrap()
});

#[allow(clippy::unwrap_used)]
static HEADER_NAME_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^:?[0-9a-zA-Z!#$%&'*+-.^_|~`]+$").unwrap());
#[allow(clippy::unwrap_used)]
static HEADER_VALUE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[^\x00-\x08\x0A-\x1F\x7F]*$").unwrap());
#[allow(clippy::unwrap_used)]
static HEADER_STRING_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^\x00\n\r]*$").unwrap());

pub trait ValidateStringExt {
    /// Validates whether the given string is a hostname
    fn validate_hostname(&self) -> anyhow::Result<()>;
    // Validates whether the given string is an email address
    fn validate_email(&self) -> anyhow::Result<()>;
    /// Validates whether the given string is an IP V4
    fn validate_ipv4(&self) -> anyhow::Result<()>;
    /// Validates whether the given string is an IP V6
    fn validate_ipv6(&self) -> anyhow::Result<()>;
    /// Validates whether the given string is an IP
    fn validate_ip(&self) -> anyhow::Result<()>;
    /// Validates whether the given string is either a valid hostname as
    /// defined by RFC 1034 (which does not support internationalized domain
    /// names or IDNs), or it can be a valid IP (v4 or v6).
    fn validate_address(&self) -> anyhow::Result<()>;
    /// Validates whether the given string is a URI
    fn validate_uri(&self) -> anyhow::Result<()>;
    /// Validates whether the given string is a URI reference
    fn validate_uri_ref(&self) -> anyhow::Result<()>;
    /// Validates whether the given string is a UUID
    fn validate_uuid(&self) -> anyhow::Result<()>;
    /// Validates whether the given string is a header name
    fn validate_header_name(&self, strict: bool) -> anyhow::Result<()>;
    /// Validates whether the given string is a header value
    fn validate_header_value(&self, strict: bool) -> anyhow::Result<()>;
}

impl<T> ValidateStringExt for T
where
    T: ToString,
{
    fn validate_hostname(&self) -> anyhow::Result<()> {
        validate_hostname(&self.to_string())
    }

    fn validate_email(&self) -> anyhow::Result<()> {
        EmailAddress::from_str(&self.to_string())
            .map(|_| ())
            .map_err(|e| format_err!("{}", e))
    }

    fn validate_ipv4(&self) -> anyhow::Result<()> {
        if IpAddr::from_str(&self.to_string()).is_ok_and(|i| i.is_ipv4()) {
            Ok(())
        } else {
            Err(format_err!("invalid ipv4 format"))
        }
    }

    fn validate_ipv6(&self) -> anyhow::Result<()> {
        if IpAddr::from_str(&self.to_string()).is_ok_and(|i| i.is_ipv6()) {
            Ok(())
        } else {
            Err(format_err!("invalid ipv6 format"))
        }
    }

    fn validate_ip(&self) -> anyhow::Result<()> {
        IpAddr::from_str(&self.to_string())
            .map(|_| ())
            .map_err(|e| format_err!("{}", e))
    }

    fn validate_address(&self) -> anyhow::Result<()> {
        if let Ok(()) = self.validate_ip() {
            return Ok(());
        }
        match self.validate_hostname() {
            Ok(()) => Ok(()),
            Err(_) => Err(format_err!("must be a valid hostname or ip address")),
        }
    }

    fn validate_uri(&self) -> anyhow::Result<()> {
        let uri = Uri::from_str(&self.to_string()).map_err(|e| format_err!("{}", e))?;
        if uri.scheme().is_some() {
            Ok(())
        } else {
            Err(format_err!("URI scheme is required"))
        }
    }

    fn validate_uri_ref(&self) -> anyhow::Result<()> {
        Uri::from_str(&self.to_string())
            .map(|_| ())
            .map_err(|e| format_err!("{}", e))
    }

    fn validate_uuid(&self) -> anyhow::Result<()> {
        if UUID_RE.is_match(&self.to_string()) {
            Ok(())
        } else {
            Err(format_err!("invalid uuid format"))
        }
    }

    fn validate_header_name(&self, strict: bool) -> anyhow::Result<()> {
        let ok = if strict {
            HEADER_NAME_RE.is_match(&self.to_string())
        } else {
            HEADER_STRING_RE.is_match(&self.to_string())
        };
        if ok {
            Ok(())
        } else {
            Err(format_err!("invalid header name"))
        }
    }

    fn validate_header_value(&self, strict: bool) -> anyhow::Result<()> {
        let ok = if strict {
            HEADER_VALUE_RE.is_match(&self.to_string())
        } else {
            HEADER_STRING_RE.is_match(&self.to_string())
        };
        if ok {
            Ok(())
        } else {
            Err(format_err!("invalid header value"))
        }
    }
}
