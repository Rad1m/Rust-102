use std::env;
use std::fs;
use std::io::Error;
use std::collections::HashMap;

struct Database {
    // Structs are STACK allocated
    // map is just a name, it can be anything
    map: HashMap<String, String>,
}

impl Database {
    // new() is a naming convention in Rust but the name can be anything, e.g. foo()
    fn new () -> Result <Database, Error> {
        let mut map = HashMap::new();
        let contents = fs::read_to_string("kv.db")?; // '?' is doing as same as the code above for handling errors
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            map.insert(key.to_owned(), value.to_owned()); // using to_owned to own borrowed string
        }
        Ok(Database { map})
    }

    // method takes self as a first argument, this is different from function
    fn insert(&mut self, key: &String, value: &String) {
        self.map.insert(key.to_string(), value.to_string());
    }

    fn flush(&self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            let kvpair = format!("{}\t{}\n", key, value);
            contents.push_str(&kvpair);
            // contents.push_str(&key);
            // contents.push('\t');
            // contents.push_str(&value);
            // contents.push('\n');
        }
        fs::write("kv.db", contents)
    }
}

fn main() {
    let mut arguments = env::args().skip(1);
    let key: String = arguments.next().unwrap();
    let value: String = arguments.next().unwrap();
    let mut database = Database::new().expect("Database::new() crashed");
    database.insert(&key.to_uppercase(), &value);
    database.insert(&key, &value);
    database.flush().unwrap();
}