use std::{
    fs,
    io::{BufReader, Read},
    path::Path,
    u8, usize,
};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let path = Path::new("src/db/test_db.db");
    let f = fs::File::open(&path)?;
    let mut reader = BufReader::new(f);

    //first 100 bytes are the file header
    let mut file_header = [0; 100];
    reader
        .read_exact(&mut file_header)
        .context("reading db file")?;

    let page_size: u16 = u16::from_be_bytes([file_header[16], file_header[17]]);
    println!("page size: {}", page_size);

    //second bytes are the page header
    let mut page = [0; 4096];
    reader.read_exact(&mut page)?;
    println!("WHAT IS IN HERE : {:?}", &page[0..15]);
    let number_of_cells: u16 = u16::from_be_bytes([page[3], page[4]]);
    println!("num of cells(tables): {number_of_cells}");

    let page_type_b = u8::from_be_bytes([page[0]]);
    println!("page type :{}", &page[0]);
    let number_of_cells: u16 = u16::from_be_bytes([page[3], page[4]]);
    println!("number of cells {number_of_cells}");

    if page_type_b > 6 {
        println!("leaf");
    } else {
        println!("inner");
    }

    let cell: u16 = u16::from_be_bytes([page[8], page[9]]);
    println!("cell : {cell:?}");

    let record = &page[cell as usize..];
    println!("record {record:?}");
    let size_of_record_header = u8::from_be(record[3]);
    println!("size of record header : {size_of_record_header:?}");
    let body = &record[3..];
    println!("body: {}", String::from_utf8_lossy(&body));

    Ok(())
}
