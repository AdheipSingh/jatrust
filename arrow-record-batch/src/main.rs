use std::sync::Arc;

use arrow::array::*;
use arrow::datatypes::*;
use arrow::error::Result as ArrowResult;
use arrow::record_batch::*;
use arrow::json;
use std::fs::File;

fn main() -> ArrowResult<()> {
   
    let batch = event1().next().unwrap().unwrap();
    println!("{:?}", batch);
    Ok(())
}

fn event1() -> json::Reader<File> {
    let file = File::open("../data/event1.json").unwrap();

    // create a builder, inferring the schema with the first 100 records
    let builder = json::ReaderBuilder::new().infer_schema(Some(100));
    let reader = builder.build::<File>(file).unwrap();

    reader
}
