use std::collections::HashMap;
fn main() {
    let mut socres = HashMap::new();
    socres.insert(String::from("blue"), 10);
    let e = socres.entry(String::from("yellow"));
    println!("{:?}", e);
    e.or_insert(50);
    socres.entry(String::from("blue")).or_insert(40);
    println!("{:?}", socres);
}

use std::fs::File;
fn main() {
    let f = File::open("hello");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open file {:?}", error);
        },
    };
}


use std::io;
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("error createing file: {:?}", error);
            })
        }else {
            panic!("error createing file: {:?}", error);
        }
    });
}




fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), 
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 简化
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 简化
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 简化2
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.f.read_to_string(&mut s)?;
    Ok(s)
}