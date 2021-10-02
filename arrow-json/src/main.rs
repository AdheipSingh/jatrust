extern crate arrow;

use arrow::json;
use std::fs::File;

fn main() {
    let s1 = event1().schema();

    let s2 = event2().schema();

    if s1 != s2 {
        println!("schema not valid")
    }

}

fn event2() -> json::Reader<File> {
    let file = File::open("../data/event2.json").unwrap();

    // create a builder, inferring the schema with the first 100 records
    let builder = json::ReaderBuilder::new().infer_schema(Some(100));
    let reader = builder.build::<File>(file).unwrap();

    reader
}

fn event1() -> json::Reader<File> {
    let file = File::open("../data/event1.json").unwrap();

    // create a builder, inferring the schema with the first 100 records
    let builder = json::ReaderBuilder::new().infer_schema(Some(100));
    let reader = builder.build::<File>(file).unwrap();

    reader
}
