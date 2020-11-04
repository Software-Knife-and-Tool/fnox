/* mu/heap.rs */
use crate::mu::r#type::{Tag, _tag_from_u8};
// use crate::mu::r#type::{ImmediateClass, _immediate_class_from_u8};

pub struct Heap {
    nwords: usize,       // number of u64 words
    fname: &'static str, // mapped file name
    mmap: Vec<u64>,      // on heap
    fence: usize,        // allocation fence, word offset
}

// use memmap;
// use std::{
//    fs::OpenOptions,
//    io::{Seek, SeekFrom, Write},
// };

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
        mmap: Vec::with_capacity(nwords * 8),
        fence: 0,
    }
}

pub fn _hinfo(reloc: u32, len: usize, refbit: u8, tag: Tag) -> u64 {
    ((reloc as u64) << 32) | ((len as u64) << 4) | ((refbit as u64) << 1) | ((tag as u64) << 0)
}

pub fn _hinfo_reloc(hinfo: u64) -> u32 {
    (hinfo >> 32) as u32
}

pub fn _hinfo_len(hinfo: u64) -> u32 {
    ((hinfo >> 60) & 0x1fffffff) as u32
}

pub fn _hinfo_refbit(hinfo: u64) -> u8 {
    ((hinfo >> 3) & 1) as u8
}

pub fn _hinfo_tag(hinfo: u64) -> Tag {
    _tag_from_u8((hinfo & 0x7) as u8)
}

impl Heap {
    pub fn alloc(&mut self, _nbytes: usize, _tag: Tag) -> u64 {
        let addr: u64 = unsafe { std::mem::transmute(&self.mmap[self.fence]) };
        let nwords: usize = (_nbytes + 15) / 8;
        let hinfo: u64 = _hinfo(0, nwords, 0, _tag);

        self.mmap[self.fence] = hinfo;
        self.fence += nwords;
        addr + 8
    }

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
