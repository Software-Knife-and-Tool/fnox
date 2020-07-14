/* mu/heap.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::entag;

pub struct Heap {
    size: u64,
    fname: &'static str
}

use memmap; // 0.7.0
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};

/*
const SIZE: u64 = 1024 * 1024;

fn mmmap(size: u64, fn: str) -> u64 {
    let src = "Hello!";

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("test.mmap")
        .expect("Unable to open file");

    -- Allocate space in the file first
    f.seek(SeekFrom::Start(SIZE)).unwrap();
    f.write_all(&[0]).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();

    let mut data = unsafe {
        memmap::MmapOptions::new()
            .map_mut(&f)
            .expect("Could not access data from memory mapped file")
    };

    data[..src.len()].copy_from_slice(src.as_bytes());
}
*/

pub fn heap(words: u64) -> Heap {
    Heap {
        size: words,
        fname: "hoopty"
    }
}

impl Heap {
    pub fn size(&self) -> u64 {
        return self.size;
    }
}

