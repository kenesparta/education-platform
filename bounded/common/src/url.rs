use crate::{Validator, ValidatorError};
use regex::Regex;
use std::fmt;
use std::ops::Deref;
use std::sync::LazyLock;
use thiserror::Error;

/// Regex for validating HTTP/HTTPS URLs.
///
/// Validates URLs with:
/// - Scheme: http or https
/// - Optional username:password authentication
/// - Host: domain name or IPv4 address
/// - Optional port
/// - Optional path, query parameters, and fragment
static URL_REGEX: LazyLock<Result<Regex, regex::Error>> = LazyLock::new(|| {
    Regex::new(
        r"^(https?://)([a-zA-Z0-9.-]+|\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})(:\d{1,5})?(/?[^\s]*)?$",
    )
});

/// Error type for `Url` validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UrlError {
    /// Validation error occurred during URL validation.
    #[error("URL validation failed: {0}")]
    ValidationError(#[from] ValidatorError),

    #[error("URL is empty")]
    EmptyValue,

    #[error("URL format is invalid")]
    FormatNotValid,

    #[error("URL scheme must be http or https")]
    SchemeNotValid,

    #[error("URL host is missing or invalid")]
    HostNotValid,

    #[error("URL is too long (max 2048 characters)")]
    TooLong,

    #[error("must start with http:// or https://")]
    StartWithHTTP,
}

/// A validated URL value object for web addresses.
///
/// `Url` is a Value Object that represents a valid HTTP or HTTPS URL.
/// It ensures the URL follows web standards and is properly formatted.
///
/// # Allowed Formats
///
/// - Scheme: `http://` or `https://` (required)
/// - Host: domain name or IPv4 address (required)
/// - Port: optional (e.g., `:8080`)
/// - Path: optional (e.g., `/path/to/page`)
/// - Query: optional (e.g., `?key=value&foo=bar`)
/// - Fragment: optional (e.g., `#section`)
///
/// # Examples
///
/// ```
/// use education_platform_common::Url;
///
/// // Simple URLs
/// let url = Url::new("https://example.com".to_string()).unwrap();
/// assert_eq!(url.as_str(), "https://example.com");
///
/// // With path and query
/// let url = Url::new("https://example.com/path?key=value".to_string()).unwrap();
/// assert_eq!(url.scheme(), "https");
///
/// // With port
/// let url = Url::new("http://localhost:8080".to_string()).unwrap();
/// assert!(url.as_str().contains(":8080"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Url {
    inner: String,
}

impl Url {
    const MAX_LENGTH: usize = 2048;

    /// Creates a new `Url` with validation.
    ///
    /// The URL will be trimmed and validated to:
    /// - Not be empty after trimming
    /// - Have a valid scheme (http or https)
    /// - Have a valid host
    /// - Be at most 2048 characters
    /// - Follow proper URL format
    ///
    /// # Errors
    ///
    /// Returns `UrlError` if validation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Url;
    ///
    /// let url = Url::new("https://example.com".to_string()).unwrap();
    /// assert_eq!(url.as_str(), "https://example.com");
    ///
    /// // Whitespace is trimmed
    /// let trimmed = Url::new("  https://example.com  ".to_string()).unwrap();
    /// assert_eq!(trimmed.as_str(), "https://example.com");
    ///
    /// // With full URL components
    /// let full = Url::new("https://example.com:443/path?query=1#fragment".to_string()).unwrap();
    /// assert!(full.as_str().contains("path"));
    /// ```
    pub fn new(url: String) -> Result<Self, UrlError> {
        let trimmed = url.trim();

        Validator::is_not_empty(trimmed)?;
        Validator::has_max_length(trimmed, Self::MAX_LENGTH)?;

        Self::is_valid_url(trimmed)?;

        Ok(Self {
            inner: trimmed.to_string(),
        })
    }

    /// Validates that a URL follows proper HTTP/HTTPS format.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Url;
    ///
    /// // Valid URLs
    /// assert!(Url::is_valid_url("https://example.com").is_ok());
    /// assert!(Url::is_valid_url("http://example.com").is_ok());
    /// assert!(Url::is_valid_url("https://example.com/path").is_ok());
    /// assert!(Url::is_valid_url("https://example.com:8080").is_ok());
    /// assert!(Url::is_valid_url("https://192.168.1.1").is_ok());
    /// assert!(Url::is_valid_url("https://example.com/path?query=1&foo=bar#section").is_ok());
    ///
    /// // Invalid URLs
    /// assert!(Url::is_valid_url("ftp://example.com").is_err());
    /// assert!(Url::is_valid_url("example.com").is_err());
    /// assert!(Url::is_valid_url("https://").is_err());
    /// assert!(Url::is_valid_url("").is_err());
    /// ```
    pub fn is_valid_url(url: &str) -> Result<(), UrlError> {
        if url.is_empty() {
            return Err(UrlError::EmptyValue);
        }

        let regex = URL_REGEX
            .as_ref()
            .map_err(|e| ValidatorError::RegexError(e.to_string()))?;

        if !regex.is_match(url) {
            return Err(UrlError::FormatNotValid);
        }

        // Additional validation: must start with http:// or https://
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(UrlError::StartWithHTTP);
        }

        Ok(())
    }

    /// Returns the URL scheme (http or https).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Url;
    ///
    /// let url = Url::new("https://example.com".to_string()).unwrap();
    /// assert_eq!(url.scheme(), "https");
    ///
    /// let url = Url::new("http://example.com".to_string()).unwrap();
    /// assert_eq!(url.scheme(), "http");
    /// ```
    #[inline]
    #[must_use]
    pub fn scheme(&self) -> &str {
        self.inner.split("://").next().unwrap_or_default()
    }

    /// Returns the host portion of the URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Url;
    ///
    /// let url = Url::new("https://example.com/path".to_string()).unwrap();
    /// assert_eq!(url.host(), "example.com");
    ///
    /// let url = Url::new("https://example.com:8080/path".to_string()).unwrap();
    /// assert_eq!(url.host(), "example.com");
    /// ```
    #[must_use]
    pub fn host(&self) -> &str {
        let without_scheme = self.inner.split("://").nth(1).unwrap_or("");
        let host_part = without_scheme.split('/').next().unwrap_or("");
        host_part.split(':').next().unwrap_or(host_part)
    }

    /// Returns true if the URL uses HTTPS.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Url;
    ///
    /// let url = Url::new("https://example.com".to_string()).unwrap();
    /// assert!(url.is_secure());
    ///
    /// let url = Url::new("http://example.com".to_string()).unwrap();
    /// assert!(!url.is_secure());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_secure(&self) -> bool {
        self.inner.starts_with("https://")
    }

    /// Returns the URL as a string slice.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Consumes the `Url` and returns the inner `String`.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String {
        self.inner
    }
}

impl Deref for Url {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<Url> for String {
    fn from(url: Url) -> Self {
        url.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn test_new_with_valid_url() {
            let result = Url::new("https://example.com".to_string());
            assert!(result.is_ok());
            assert_eq!(result.unwrap().as_str(), "https://example.com");
        }

        #[test]
        fn test_new_trims_whitespace() {
            let result = Url::new("  https://example.com  ".to_string());
            assert!(result.is_ok());
            assert_eq!(result.unwrap().as_str(), "https://example.com");
        }

        #[test]
        fn test_new_with_empty_string_returns_error() {
            let result = Url::new("".to_string());
            assert!(result.is_err());
        }

        #[test]
        fn test_new_with_whitespace_only_returns_error() {
            let result = Url::new("   ".to_string());
            assert!(result.is_err());
        }

        #[test]
        fn test_new_with_http_scheme() {
            let result = Url::new("http://example.com".to_string());
            assert!(result.is_ok());
        }

        #[test]
        fn test_new_with_https_scheme() {
            let result = Url::new("https://example.com".to_string());
            assert!(result.is_ok());
        }

        #[test]
        fn test_new_with_invalid_scheme_returns_error() {
            let result = Url::new("ftp://example.com".to_string());
            assert!(result.is_err());
        }

        #[test]
        fn test_new_without_scheme_returns_error() {
            let result = Url::new("example.com".to_string());
            assert!(result.is_err());
        }

        #[test]
        fn test_new_with_too_long_url_returns_error() {
            let long_url = format!("https://example.com/{}", "a".repeat(2048));
            let result = Url::new(long_url);
            assert!(result.is_err());
        }
    }

    mod validation {
        use super::*;

        #[test]
        fn test_is_valid_url_with_https() {
            assert!(Url::is_valid_url("https://example.com").is_ok());
        }

        #[test]
        fn test_is_valid_url_with_http() {
            assert!(Url::is_valid_url("http://example.com").is_ok());
        }

        #[test]
        fn test_is_valid_url_with_subdomain() {
            assert!(Url::is_valid_url("https://sub.example.com").is_ok());
            assert!(Url::is_valid_url("https://deep.sub.example.com").is_ok());
        }

        #[test]
        fn test_is_valid_url_with_path() {
            assert!(Url::is_valid_url("https://example.com/path").is_ok());
            assert!(Url::is_valid_url("https://example.com/path/to/page").is_ok());
        }

        #[test]
        fn test_is_valid_url_with_query() {
            assert!(Url::is_valid_url("https://example.com?query=value").is_ok());
            assert!(Url::is_valid_url("https://example.com?a=1&b=2").is_ok());
        }

        #[test]
        fn test_is_valid_url_with_fragment() {
            assert!(Url::is_valid_url("https://example.com#section").is_ok());
            assert!(Url::is_valid_url("https://example.com/path#section").is_ok());
        }

        #[test]
        fn test_is_valid_url_with_port() {
            assert!(Url::is_valid_url("https://example.com:443").is_ok());
            assert!(Url::is_valid_url("http://localhost:8080").is_ok());
        }

        #[test]
        fn test_is_valid_url_with_ip_address() {
            assert!(Url::is_valid_url("https://192.168.1.1").is_ok());
            assert!(Url::is_valid_url("http://127.0.0.1:8080").is_ok());
        }

        #[test]
        fn test_is_valid_url_full_url() {
            assert!(
                Url::is_valid_url("https://example.com:443/path?query=1&foo=bar#section").is_ok()
            );
        }

        #[test]
        fn test_is_valid_url_with_hyphen_in_domain() {
            assert!(Url::is_valid_url("https://my-domain.com").is_ok());
        }

        #[test]
        fn test_is_valid_url_invalid_scheme() {
            assert!(Url::is_valid_url("ftp://example.com").is_err());
            assert!(Url::is_valid_url("file:///path").is_err());
        }

        #[test]
        fn test_is_valid_url_no_scheme() {
            assert!(Url::is_valid_url("example.com").is_err());
            assert!(Url::is_valid_url("www.example.com").is_err());
        }

        #[test]
        fn test_is_valid_url_scheme_only() {
            assert!(Url::is_valid_url("https://").is_err());
            assert!(Url::is_valid_url("http://").is_err());
        }

        #[test]
        fn test_is_valid_url_empty() {
            assert!(Url::is_valid_url("").is_err());
        }
    }

    mod getters {
        use super::*;

        #[test]
        fn test_scheme_returns_https() {
            let url = Url::new("https://example.com".to_string()).unwrap();
            assert_eq!(url.scheme(), "https");
        }

        #[test]
        fn test_scheme_returns_http() {
            let url = Url::new("http://example.com".to_string()).unwrap();
            assert_eq!(url.scheme(), "http");
        }

        #[test]
        fn test_host_returns_domain() {
            let url = Url::new("https://example.com".to_string()).unwrap();
            assert_eq!(url.host(), "example.com");
        }

        #[test]
        fn test_host_with_subdomain() {
            let url = Url::new("https://sub.example.com".to_string()).unwrap();
            assert_eq!(url.host(), "sub.example.com");
        }

        #[test]
        fn test_host_with_port() {
            let url = Url::new("https://example.com:8080".to_string()).unwrap();
            assert_eq!(url.host(), "example.com");
        }

        #[test]
        fn test_host_with_path() {
            let url = Url::new("https://example.com/path".to_string()).unwrap();
            assert_eq!(url.host(), "example.com");
        }

        #[test]
        fn test_host_ip_address() {
            let url = Url::new("https://192.168.1.1".to_string()).unwrap();
            assert_eq!(url.host(), "192.168.1.1");
        }

        #[test]
        fn test_is_secure_true_for_https() {
            let url = Url::new("https://example.com".to_string()).unwrap();
            assert!(url.is_secure());
        }

        #[test]
        fn test_is_secure_false_for_http() {
            let url = Url::new("http://example.com".to_string()).unwrap();
            assert!(!url.is_secure());
        }
    }

    mod real_world_examples {
        use super::*;

        #[test]
        fn test_website_urls() {
            assert!(Url::new("https://www.google.com".to_string()).is_ok());
            assert!(Url::new("https://github.com".to_string()).is_ok());
            assert!(Url::new("https://stackoverflow.com".to_string()).is_ok());
        }

        #[test]
        fn test_api_endpoints() {
            assert!(Url::new("https://api.example.com/v1/users".to_string()).is_ok());
            assert!(Url::new("https://api.github.com/repos".to_string()).is_ok());
        }

        #[test]
        fn test_urls_with_query_params() {
            assert!(Url::new("https://example.com/search?q=rust&lang=en".to_string()).is_ok());
            assert!(Url::new("https://example.com?page=1&limit=10".to_string()).is_ok());
        }

        #[test]
        fn test_urls_with_fragments() {
            assert!(Url::new("https://example.com/docs#installation".to_string()).is_ok());
            assert!(Url::new("https://example.com#top".to_string()).is_ok());
        }

        #[test]
        fn test_localhost_urls() {
            assert!(Url::new("http://localhost:3000".to_string()).is_ok());
            assert!(Url::new("http://127.0.0.1:8080".to_string()).is_ok());
        }

        #[test]
        fn test_course_resource_urls() {
            assert!(Url::new("https://example.com/courses/rust-101".to_string()).is_ok());
            assert!(
                Url::new("https://example.com/videos/intro.mp4?quality=hd".to_string()).is_ok()
            );
        }
    }

    mod value_object_semantics {
        use super::*;

        #[test]
        fn test_equality() {
            let url1 = Url::new("https://example.com".to_string()).unwrap();
            let url2 = Url::new("https://example.com".to_string()).unwrap();
            assert_eq!(url1, url2);
        }

        #[test]
        fn test_inequality() {
            let url1 = Url::new("https://example.com".to_string()).unwrap();
            let url2 = Url::new("https://different.com".to_string()).unwrap();
            assert_ne!(url1, url2);
        }

        #[test]
        fn test_clone() {
            let url1 = Url::new("https://example.com".to_string()).unwrap();
            let url2 = url1.clone();
            assert_eq!(url1, url2);
        }

        #[test]
        fn test_hash_consistency() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            let url1 = Url::new("https://example.com".to_string()).unwrap();
            let url2 = Url::new("https://example.com".to_string()).unwrap();
            set.insert(url1);
            assert!(set.contains(&url2));
        }

        #[test]
        fn test_ordering() {
            let url1 = Url::new("https://aaa.com".to_string()).unwrap();
            let url2 = Url::new("https://bbb.com".to_string()).unwrap();
            assert!(url1 < url2);
            assert!(url2 > url1);
        }

        #[test]
        fn test_debug_format() {
            let url = Url::new("https://example.com".to_string()).unwrap();
            let debug = format!("{:?}", url);
            assert!(debug.contains("Url"));
        }

        #[test]
        fn test_display_format() {
            let url = Url::new("https://example.com".to_string()).unwrap();
            assert_eq!(format!("{}", url), "https://example.com");
        }

        #[test]
        fn test_deref() {
            let url = Url::new("https://example.com".to_string()).unwrap();
            assert_eq!(url.len(), 19);
            assert!(url.starts_with("https"));
        }

        #[test]
        fn test_as_ref() {
            let url = Url::new("https://example.com".to_string()).unwrap();
            let s: &str = url.as_ref();
            assert_eq!(s, "https://example.com");
        }

        #[test]
        fn test_into_string() {
            let url = Url::new("https://example.com".to_string()).unwrap();
            let string: String = url.into();
            assert_eq!(string, "https://example.com");
        }

        #[test]
        fn test_into_inner() {
            let url = Url::new("https://example.com".to_string()).unwrap();
            let string = url.into_inner();
            assert_eq!(string, "https://example.com");
        }
    }

    mod error_handling {
        use super::*;

        #[test]
        fn test_empty_error() {
            let result = Url::new("".to_string());
            assert!(matches!(result, Err(UrlError::ValidationError(_))));
        }

        #[test]
        fn test_format_error() {
            let result = Url::new("not-a-url".to_string());
            assert!(matches!(result, Err(UrlError::FormatNotValid)));
        }

        #[test]
        fn test_validation_error_for_too_long() {
            let long_url = format!("https://example.com/{}", "a".repeat(2048));
            let result = Url::new(long_url);
            assert!(matches!(result, Err(UrlError::ValidationError(_))));
        }
    }
}
