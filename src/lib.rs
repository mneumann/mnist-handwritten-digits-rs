extern crate byteorder;

use std::path::Path;
use std::io::{self, Read};
use std::fs::File;
use byteorder::{ReadBytesExt, BigEndian};

pub fn read_label_file<T:AsRef<Path>>(filename: T) -> io::Result<Vec<u8>> {
    let mut f = try!(File::open(filename));
    let magic_nr = try!(f.read_u32::<BigEndian>());
    assert!(magic_nr == 0x0801);
    let nelems = try!(f.read_u32::<BigEndian>()) as usize;
    assert!(nelems > 0);
    let mut labels = Vec::with_capacity(nelems);
    //unsafe { labels.set_len(nelems); }
    //try!(f.read_exact(&labels[..]));
    for byte in f.bytes().take(nelems) {
        let lbl = try!(byte);
        assert!(lbl <= 9);
        labels.push(lbl);
    }

    assert!(labels.len() == nelems);

    return Ok(labels);
}
