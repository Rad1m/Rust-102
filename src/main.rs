use std::env;
use std::fs;
use std::io::Error;
use std::collections::HashMap;

struct Database {
    // Structs are STACK allocated
    // map is just a name, it can be anything
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    // new() is a naming convention in Rust but the name can be anything, e.g. foo()
    fn new () -> Result <Database, Error> {
        // TO DO:
        // 1. read kv.db file
        // 2. parse the string
        // 3. populate the map

        // let contents = match fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Result::Err(error);
        //     }
        // };
        let contents = fs::read_to_string("kv.db")?; // '?' is doing as same as the code above for handling errors
        let mut map = HashMap::new();
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            println!("The key is {} and value is {}", key, value);
            map.insert(key.to_owned(), value.to_owned()); // using to_owned to own borrowed string
        }
        
        
        Ok(Database { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) {
            self.map.insert(key, value);
        }

    fn flush(&self) -> std::io::Result<()> {
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    println!("Do flush called");
    let mut contents = String::new();
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(&&&&value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}

fn main() {
    let mut arguments = env::args().skip(1);
    let key: String = arguments.next().unwrap();
    let value: String = arguments.next().unwrap();
    // println!("The key is {} and value is {}", key, value);

    // written by me as a test
    // let message = format!("This is a test file.\nThe key is {} and value is {}.\n", key, value);
    // fs::write("foo.txt", message).expect("Unable to write file");
    
    let contents = format!("{}\t{}\n", key, value);
    fs::write("kv.db", contents).expect("Unable to write file");

    let database = Database::new().expect("Database::new() crashed");
}
