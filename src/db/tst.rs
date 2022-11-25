#[cfg(test)]
mod tests{
    use crate::db::*;
    use anyhow::Result;

    #[test]
    fn save_load_command_glob() ->Result<()>{
        let alias = "testonlyasdfasadf".to_string();
        let command = "echo 'hi'".to_string();
        if exists(&alias, &StorageMode::Global){
            remove_alias(&alias, &StorageMode::Global)?;
        }
        save(&alias, &command, &StorageMode::Global)?;
        assert_eq!(read(&alias, &StorageMode::Global)?, command);
        remove_alias(&alias, &StorageMode::Global)?;
        Ok(())
    }
}