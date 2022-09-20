/* this script compresses and stores data in memory in tabular and graph format */

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::LinkedList;
use std::collections::BinaryHeap;

// importing compression algorithms
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::write::DeflateEncoder;
use flate2::read::DeflateDecoder;

// importing crates for interacting with the memory and gpu
use std::mem;
use std::ptr;
use std::slice;

// importing crates for interacting with the file system, we might need to write to the extended ram stored on the hard drive
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::SeekFrom;

// importing crates for interacting with the operating system
use std::env;
use std::process;
use std::thread;

// defining the function to compress data
fn compress_data(data: &[u8], compression_type: &str) -> Vec<u8> {
    let mut compressed_data = Vec::new();
    match compression_type {
        "zlib" => {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data).unwrap();
            compressed_data = encoder.finish().unwrap();
        },
        "gzip" => {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data).unwrap();
            compressed_data = encoder.finish().unwrap();
        },
        "deflate" => {
            let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data).unwrap();
            compressed_data = encoder.finish().unwrap();
        },
        _ => {
            println!("Error: compression type not supported");
            process::exit(1);
        }
    }
    return compressed_data;
}

// defining the function to decompress data
fn decompress_data(data: &[u8], compression_type: &str) -> Vec<u8> {
    let mut decompressed_data = Vec::new();
    match compression_type {
        "zlib" => {
            let mut decoder = ZlibDecoder::new(data);
            decoder.read_to_end(&mut decompressed_data).unwrap();
        },
        "gzip" => {
            let mut decoder = GzDecoder::new(data);
            decoder.read_to_end(&mut decompressed_data).unwrap();
        },
        "deflate" => {
            let mut decoder = DeflateDecoder::new(data);
            decoder.read_to_end(&mut decompressed_data).unwrap();
        },
        _ => {
            println!("Error: compression type not supported");
            process::exit(1);
        }
    }
    return decompressed_data;
}

// defining the function to store data in memory
fn store_data_in_memory(data: &[u8], compression_type: &str) -> Vec<u8> {
    let compressed_data = compress_data(data, compression_type);
    let compressed_data_size = compressed_data.len();
    let compressed_data_pointer = compressed_data.as_ptr();
    let compressed_data_slice = unsafe { slice::from_raw_parts(compressed_data_pointer, compressed_data_size) };
    let mut compressed_data_vector = Vec::new();
    compressed_data_vector.extend_from_slice(compressed_data_slice);
    return compressed_data_vector;
}

// defining the function to retrieve data from memory
fn retrieve_data_from_memory(data: &[u8], compression_type: &str) -> Vec<u8> {
    let decompressed_data = decompress_data(data, compression_type);
    let decompressed_data_size = decompressed_data.len();
    let decompressed_data_pointer = decompressed_data.as_ptr();
    let decompressed_data_slice = unsafe { slice::from_raw_parts(decompressed_data_pointer, decompressed_data_size) };
    let mut decompressed_data_vector = Vec::new();
    decompressed_data_vector.extend_from_slice(decompressed_data_slice);
    return decompressed_data_vector;
}

// defining the function to store data in the vram of the gpu
//TODO: implement this function

// main function to compress a tabular dataset, store it in memory, and send back a pointer to retrieve it on demand
fn compressor(data: &[u8], compression_type: &str) -> Vec<u8> {
    let compressed_data_vec = store_data_in_memory(data, compression_type);
    return compressed_data_vec;
}

// main function to use a pointer to retrieve a tabular dataset from memory, then decompress it and send it back in tabular format
fn decompressor(data: &[u8], compression_type: &str) -> Vec<u8> {
    let decompressed_data_vec = retrieve_data_from_memory(data, compression_type);
    return decompressed_data_vec;
}

