use std::io;
use std::io::{BufReader,BufRead};
use std::fs::File;

use byteorder::{LittleEndian,ReadBytesExt};

fn main() -> io::Result<()> {
    let file = File::open("foo.txt")?;
    let mut reader = BufReader::new(file);

    // read 3 integers i32, but doesn't work with Vec
    let mut array = [0; 3];

    loop {
        let mut buffer = reader.fill_buf()?;

        let length = buffer.len();
        if length == 0 { break; }

        buffer.read_i32_into::<LittleEndian>(&mut array)?;

        reader.consume(length);
    }
    println!("{:?}", array);

    // this works for reading 1 integer
    // let array = buffer.read_i32::<LittleEndian>()?;

    Ok(())
}
