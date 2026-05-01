use std::fs;
use regex::Regex;

pub fn migrate(){
    let connection_str = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");
    let connection = rusqlite::Connection::open(connection_str).unwrap();
}

pub fn load_migration() -> Vec<String>{
    let path = "../folio-desktop/migrations/category_group_column.sql";
    let data = fs::read_to_string(path).unwrap();
    let mut in_block = false;
    let mut blocks = vec![];
    let mut block = String::new();
    // TODO: seek until next block or EOF
    for line in data.lines(){
        let stripped_line = line.replace(" ","");
        if stripped_line == "--migrate:up" || stripped_line == "--migrate:down"{
            if !in_block{
                in_block = true;
                continue;
            }

            blocks.push(block.clone());
            in_block = false
        }

        if in_block{
            block.push_str(line)
        }
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_migration();
    }
}
