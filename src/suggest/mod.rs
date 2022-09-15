use crate::db;
use anyhow::{Result, Context, anyhow};

pub fn suggest_alias(command: &String) -> Result<String>{
    // Suggests an alias based on the given command
    // Gives an alias that doesn't exist
    let mut words: Vec<&str> = command.split(&[' ', '-'][..]).collect();
    let mut alias_words = vec![String::new(); words.len()];
    let mut i = 0;
    let mut empcnt = 0; // empty count
    while empcnt<words.len(){
        if words[i].is_empty(){
            empcnt+=1;
            continue;
        }
        alias_words[i].push_str(&words[i][0..1]);
        let alias = alias_words.join("");
        if alias.len()>=words.len() && !db::exists(&alias){
            return Ok(alias);
        }
        words[i] = &words[i][1..];
        i = (i+1)%words.len();
    }
    Err(anyhow!("No suggestions found"))
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn make_sug() -> Result<()>{
        let sug = suggest_alias(&"git push origin master".to_string())?;
        println!("suggestion: {}", sug);
        Ok(())
    }
}