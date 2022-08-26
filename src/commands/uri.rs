use super::{QaliCommand};
use anyhow::{Context};
use regex::Regex;

#[derive(Debug)]
pub struct Uri{
    pub uri: String,
    pub application: Option<String>
}

// Open URI with default application
impl QaliCommand for Uri{
    fn execute(&self, args: Option<&String>) -> anyhow::Result<()> {
        eprintln!("running URI");
        let mut app = self.application.clone();
        if let Some(arg) = args{
            app = Some(arg.to_string());
        }
        match app {
            Some(app) => open::with(&self.uri, app),
            None => open::that(&self.uri)
        }.context("Failed to open URI!")?;
        Ok(())
    }

    fn is_valid(command: &String) -> bool where Self: Sized {
        let re = Regex::new(r".*://.*").unwrap();
        re.is_match(command)
    }
    
    fn new(command: &String) -> anyhow::Result<Self> where Self: Sized {
        Ok(Uri{
            uri: command.to_string(),
            application: None,
        })
    }

    fn export(&self) -> anyhow::Result<String> {
        Ok(self.uri.clone())
    }
}