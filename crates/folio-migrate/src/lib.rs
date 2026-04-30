use std::fs;

pub fn load_migration(){
    // TODO: find the first place with -- migrate:up
    let path = "../folio-desktop/migrations/category_group.sql";
    let data = fs::read_to_string(path).unwrap();
    dbg!(data);
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
