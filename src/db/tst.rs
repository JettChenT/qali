#[cfg(test)]
mod tests{
    use crate::db::*;

    #[test]
    fn save_load_command() -> io::Result<()>{
        let alias = "testonlyasdfasadf".to_string();
        let command = "echo 'hi'".to_string();
        if exists(&alias){
            assert_eq!(remove_alias(&alias), Ok(()));
        }
        save(&alias, &command)?;
        assert_eq!(read(&alias)?, command);
        assert_eq!(remove_alias(&alias), Ok(()));
        Ok(())
    }
}