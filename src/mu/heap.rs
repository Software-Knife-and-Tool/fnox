/* mu/heap.rs */
// use crate::mu::r#type::Tag;
// use crate::mu::r#type::Type;
// use crate::mu::r#type::entag;

pub struct Heap {
    nwords: u32,           // number of u64 words
    fname: &'static str,   // mapped file name
    mmap: memmap::MmapMut, // mapped file segment
    fence: u32             // allocation fence
}

use memmap;
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};

fn mmap(size: u64, fname: &str) -> memmap::MmapMut {
    let mut map =
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(fname)
            .expect("unable to create heap image (image file)");

    map.seek(SeekFrom::Start(size)).unwrap();
    map.write_all(&[0]).unwrap();
    map.seek(SeekFrom::Start(0)).unwrap();

    // let src = "Hello!";
    // let mut data = 
    //    unsafe {
    //        memmap::MmapOptions::new()
    //            .map_mut(&f)
    //            .expect("could not access data from memory mapped file")
    //    }
    //
    // data[..src.len()].copy_from_slice(src.as_bytes());

    unsafe {
        memmap::MmapOptions::new()
            .map_mut(&map)
            .expect("unable not create heap image (mmap)")
    }
}

pub fn _heap(nwords: u32) -> Heap {
    Heap {
        nwords,
        fname: "/var/tmp/lispox",
        mmap: mmap((nwords * 8).into(), "/var/tmp/lispox"),
        fence: 0
    }
}

impl Heap {
    pub fn size(&self) -> u64 {
        (self.nwords * 8).into()
    }
    pub fn file_name(&self) -> &str {
        self.fname
    }
    pub fn fence(&self) -> u32 {
        self.fence
    }
}

