//! This DNS provider provides an example implementation of a Ferron DNS provider for ACME.
//! It demonstrates how to create a mock DNS provider for testing purposes

use std::{collections::HashMap, error::Error};

use async_trait::async_trait;

use ferron_common::dns::DnsProvider;

/// Mock DNS provider
pub struct MockDnsProvider;

impl MockDnsProvider {
  /// Load a mock DNS provider from ACME challenge parameters
  pub fn from_parameters(_challenge_params: &HashMap<String, String>) -> Result<Self, Box<dyn Error + Send + Sync>> {
    // No parameters needed
    Ok(Self)
  }
}

#[async_trait]
impl DnsProvider for MockDnsProvider {
  async fn set_acme_txt_record(
    &self,
    acme_challenge_identifier: &str,
    dns_value: &str,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    // We're just printing the DNS record value
    println!("DNS record value:");
    println!("_acme-challenge.{} IN TXT {}", acme_challenge_identifier, dns_value);
    Ok(())
  }
}
