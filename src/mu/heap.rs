/* mu/heap.rs */
// use crate::mu::r#type::Tag;
// use crate::mu::r#type::Type;
// use crate::mu::r#type::entag;

pub struct Heap {
    size: u64,
    fname: &'static str,
    mmap: memmap::MmapMut
}

use memmap; // 0.7.0
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};

fn mmap(size: u64, fname: &str) -> memmap::MmapMut {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(fname)
        .expect("Unable to open file");

    // Allocate space in the file first
    f.seek(SeekFrom::Start(size)).unwrap();
    f.write_all(&[0]).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();

    // let src = "Hello!";
    // let mut data = 
    //    unsafe {
    //        memmap::MmapOptions::new()
    //            .map_mut(&f)
    //            .expect("Could not access data from memory mapped file")
    //    }
    //
    // data[..src.len()].copy_from_slice(src.as_bytes());

    unsafe {
        memmap::MmapOptions::new()
            .map_mut(&f)
            .expect("Could not access data from memory mapped file")
    }
}

pub fn heap(words: u64) -> Heap {
    println!("making heap, damnit");
    Heap {
        size: words,
        fname: "hoopty",
        mmap: mmap(1024 * 1024, "hoopty")
    }
}

impl Heap {
    pub fn size(&self) -> u64 {
        return self.size;
    }
}

