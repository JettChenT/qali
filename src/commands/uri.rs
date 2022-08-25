use super::{Execute};
use anyhow::{Context};

pub struct Uri{
    pub uri: String,
    pub application: Option<String>
}

// Open URI with default application
impl Execute for Uri{
    fn execute(&self, args: Option<&String>) -> anyhow::Result<()> {
        match &self.application {
            Some(app) => open::with(&self.uri, app),
            None => open::that(&self.uri)
        }.context("Failed to open URI!")?;
        Ok(())
    }
}