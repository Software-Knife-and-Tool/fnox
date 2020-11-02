/* mu/heap.rs */
use crate::mu::r#type::{Tag};

pub struct Heap<'a> {
    nwords: usize,         // number of u64 words
    fname: &'static str,   // mapped file name
    // mmap: memmap::MmapMut, // mapped file segment
    mmap: &'a mut Vec<u8>,         // on heap
    fence: usize           // allocation fence
}

use memmap;
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};

fn alloc(mut heap: &Heap, nwords: u16, tag: Tag) -> u64 {
    let addr: u64 = unsafe { std::mem::transmute(b"1234") };
    addr
}

/*
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

    unsafe {
        memmap::MmapOptions::new()
            .map_mut(&map)
            .expect("unable to create heap image (mmap)")
    }
}
*/

pub fn _heap(nwords: usize) -> Heap {
    Heap {
        nwords,
        fname: "/var/tmp/lispox",
        // mmap: mmap((nwords * 8).into(), "/var/tmp/lispox"),
        mmap: Vec::with_capacity(nwords * 8),
        fence: 0
    }
}

impl Heap {
    pub fn size(&self) -> usize {
        self.nwords
    }
    pub fn file_name(&self) -> &str {
        self.fname
    }
    pub fn next(&self) -> usize {
        self.fence
    }
}

