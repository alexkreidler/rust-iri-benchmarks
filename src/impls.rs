use crate::api::BenchIRI;
use anyhow::{anyhow, Error, Result};

pub use iref::Iri;
pub use iri_string::types::IriStr;
pub use sophia_iri::resolve::IriParsed;

impl<'a> BenchIRI for Iri<'a> {
    fn create_from_str(data: &'static str) -> Result<()> {
        Self::new(data.as_bytes()).or_else(|_| Err(anyhow!("failed to parse")))?;
        Ok(())
    }
}

impl<'a> BenchIRI for IriParsed<'a> {
    fn create_from_str(data: &'static str) -> Result<()> {
        Self::new(data).or_else(|_| Err(anyhow!("failed to parse")))?;
        Ok(())
    }
}

impl BenchIRI for IriStr {
    fn create_from_str(data: &'static str) -> Result<()> {
        Self::new(data).or_else(|_| Err(anyhow!("failed to parse")))?;
        Ok(())
    }
}
