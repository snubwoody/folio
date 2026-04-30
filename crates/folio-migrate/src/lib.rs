use std::fs;
use regex::Regex;

pub fn load_migration(){
    // TODO: test missing
    let regex = Regex::new("(?m)^--\\s*migrate:up(\\s*$|\\s+\\S+)")
        .expect("failed to compile regex");
    // TODO: find the first place with -- migrate:up
    let path = "../folio-desktop/migrations/category_group_column.sql";
    let data = fs::read_to_string(path).unwrap();
    for section in regex.find_iter(&data){
        dbg!(section);
    }
    // dbg!(data);
}

pub fn read_migrations(){
    for entry in fs::read_dir("../folio-desktop/migrations").unwrap(){
        dbg!(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_migration();
    }
}
