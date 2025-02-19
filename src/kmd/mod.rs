use crate::error::AlgonautError;
use algonaut_client::kmd::v1::Client;

pub mod v1;

/// KmdBuilder is the entry point to the creation of a client for the Algorand key management daemon.
/// ```
/// use algonaut::kmd::KmdBuilder;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let algod = KmdBuilder::new()
///         .bind("http://localhost:4001")
///         .auth("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
///         .build_v1()?;
///
///     println!("Algod versions: {:?}", algod.versions().await?.versions);
///
///     Ok(())
/// }
/// ```
pub struct KmdBuilder<'a> {
    url: Option<&'a str>,
    token: Option<&'a str>,
}

impl<'a> KmdBuilder<'a> {
    /// Start the creation of a client.
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind to a URL.
    pub fn bind(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    /// Use a token to authenticate.
    pub fn auth(mut self, token: &'a str) -> Self {
        self.token = Some(token);
        self
    }

    /// Build a v1 client for Algorand protocol daemon.
    ///
    /// Returns an error if url or token is not set or has an invalid format.
    pub fn build_v1(self) -> Result<v1::Kmd, AlgonautError> {
        match (self.url, self.token) {
            (Some(url), Some(token)) => Ok(v1::Kmd::new(Client::new(url, token)?)),
            (None, Some(_)) => Err(AlgonautError::UnitializedUrl),
            (Some(_), None) => Err(AlgonautError::UnitializedToken),
            (None, None) => Err(AlgonautError::UnitializedUrl),
        }
    }
}

impl<'a> Default for KmdBuilder<'a> {
    fn default() -> Self {
        KmdBuilder {
            url: None,
            token: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_client_builder() {
        let algod = KmdBuilder::new()
            .bind("http://example.com")
            .auth("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
            .build_v1();

        assert!(algod.ok().is_some());
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_client_builder_with_no_token() {
        let _ = KmdBuilder::new()
            .bind("http://example.com")
            .build_v1()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_client_builder_with_no_url() {
        let _ = KmdBuilder::new()
            .auth("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
            .build_v1()
            .unwrap();
    }
}
