use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Test1 {
    a: String,
    b: String,
    c: String,
    #[serde(alias = "type")]
    type_str: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Test2 {
    Foo { a: String, b: String, c: String },
    Bar { a: String, b: String, c: String },
}

fn main() {
    // Works without issues
    let file = File::open("test.csv").unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    for record in rdr.deserialize() {
        let result: Test1 = record.unwrap();
        println!("{:?}", result);
    }

    // Doesn't work
    let file = File::open("test.csv").unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    for record in rdr.deserialize() {
        let result: Test2 = record.unwrap();
        println!("{:?}", result);
    }
}
