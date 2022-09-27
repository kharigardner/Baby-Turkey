/* this is the backend to store the data entities of the database in columnar format,
it is accomplished with the help of Apache Arrow and Apache Parquet */

// importing the required libraries
use arrow::array::{Array, ArrayRef, BinaryArray, Float64Array, Int64Array, StringArray, TimestampNanosecondArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::error::Result as ArrowResult;
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;
use parquet::file::writer::InMemoryWriteableCursor;
use std::fs::File;
use std::io::Cursor;
use std::io::SeekFrom;
use std::io::Write;

// defining the function to create a schema for the data entities
pub fn create_schema(data_type: &str) -> Schema {
    let schema = match data_type {
        "int" => {
            let field = Field::new("int", DataType::Int64, false);
            Schema::new(vec![field])
        },
        "float" => {
            let field = Field::new("float", DataType::Float64, false);
            Schema::new(vec![field])
        },
        "string" => {
            let field = Field::new("string", DataType::Utf8, false);
            Schema::new(vec![field])
        },
        "timestamp" => {
            let field = Field::new("timestamp", DataType::TimestampNanosecond, false);
            Schema::new(vec![field])
        },
        "binary" => {
            let field = Field::new("binary", DataType::Binary, false);
            Schema::new(vec![field])
        },
        _ => {
            panic!("Error: data type not supported");
        }
    };
    return schema;
}

// defining the function to create a record batch for the data entities
pub fn create_record_batch()

// defining the function to write the data entities to the disk
pub fn entity_writer()

