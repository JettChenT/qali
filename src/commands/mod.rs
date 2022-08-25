use anyhow::Result;
pub mod python;
pub mod command;
pub mod shell;
pub mod uri;

pub enum ExecutionTypes {
    Command,
    Python,
    Shell,
    Uri,
}

pub trait Execute {
    fn execute(&self, args: Option<&String>) -> Result<()>;
}
