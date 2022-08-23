#[cfg(test)]
mod tests{
    use crate::db::*;
    use anyhow::Result;

    #[test]
    fn save_load_command() ->Result<()>{
        let alias = "testonlyasdfasadf".to_string();
        let command = "echo 'hi'".to_string();
        if exists(&alias){
            remove_alias(&alias)?;
        }
        save(&alias, &command)?;
        assert_eq!(read(&alias)?, command);
        remove_alias(&alias)?;
        Ok(())
    }
}