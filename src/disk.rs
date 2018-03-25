extern crate config;

/// A representation of a disk.
///
/// All IO to the disk is handled by this module
///
///
use std::str;

use std::io;
use std::io::Read;

use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::fs::File;
use std::fs::OpenOptions;

use std::io::SeekFrom;

use std::process::Command;
use chrono::prelude::*;

use serde_json;

use config::Config;
use header::Header;

// Constants
const SIGNATURE: u64 = 0x0510D05E;
const VERSION: u64 = 1;

#[derive(Debug)]
pub struct Disk {
    diskname: String,  // Disk name
    disksize: u64,     // Disk physcal size in bytes
    file: File,        // Pointer to the file
    version: u64,      // Version - should be large enough
    create: i64,       // created timestamp
    headeroffset: u64, // Offset to the graph header
    headersize: u64,   // size of the graph header
    headerspace: u64,  // max size of the graph header
    header: Header,    // graph header
}

impl Disk {
    pub fn new<'a>(config: &Config) -> Result<Disk, io::Error> {
        let h: String = config.get("hostname").unwrap();
        let n = format!("disk_{}", h);
        let diskname: String = {
            match config.get(&n) {
                Ok(n1) => n1,
                Err(_) => match config.get("disk_default") {
                    Ok(n2) => n2,
                    Err(_err) => return Err(Error::new(ErrorKind::Other, "No disk in config")),
                },
            }
        };
        debug!("Creating structures for disk {} ", diskname);

        let mut file = OpenOptions::new().read(true).write(true).open(&diskname)?;

        // Get the disk size

        let output = {
            Command::new("fdisk")
                .arg("-l")
                .arg(&diskname)
                .output()
                .expect("Failed")
        };
        let s = String::from_utf8_lossy(&output.stdout);
        let v: Vec<&str> = s.split(" ").collect();
        let disksize = match v[4].parse::<u64>() {
            Ok(v) => v,
            Err(err) => return Err(Error::new(ErrorKind::Other, err.to_string())),
        };
        debug!("Size = <{}>", disksize);

        // Read the signature
        let sig: u64 = file.read_u64()?;

        if SIGNATURE != sig {
            // Init the disk here
            warn!(
                "Disk header signature invalid. Got={:X} expected={:X}",
                sig, SIGNATURE
            );
            warn!("Initializing disk");

            let utc: DateTime<Utc> = Utc::now();
            trace!("UTC = {:?}", utc);

            let headerspace: u64 = match config.get_int("HEADER_SPACE") {
                Ok(n2) => n2 as u64,
                Err(_err) => return Err(Error::new(ErrorKind::Other, "No disk in config")),
            };
            trace!("Header space = {} ", headerspace);

            let diskheadersize = 48; // 6 u/i64 values * 8 butes = 48

            let header = Header::new(diskheadersize, disksize, headerspace);
            let headerbytes = serde_json::to_string(&header)?;
            let headersize: u64 = headerbytes.len() as u64;
            trace!("Disk header {:?}", headerbytes);

            trace!("Starting to write header");

            file.seek(SeekFrom::Start(0))?;
            file.write_u64(SIGNATURE)?;
            trace!("Wrote signature ");
            file.write_u64(VERSION)?;
            file.write_i64(utc.timestamp())?;
            file.write_u64(diskheadersize)?;
            file.write_u64(headersize)?;
            file.write_u64(headerspace)?;

            trace!("Starting to write graph header");
            file.seek(SeekFrom::Start(diskheadersize))?;

            trace!("Seeked to {}", diskheadersize);
            file.write_string(headerbytes)?;

            // Reset read pointer to just after signature
            trace!("Reset file pointer to re-read heder");
            file.seek(SeekFrom::Start(8))?;
            trace!("Disk should now be initialised");
        } else {
            debug!(
                "Disk header signature is valid. Got={:X} expected={:X}",
                sig, SIGNATURE
            );
        }

        // Read the disk header
        info!("Reading disk header");

        let version: u64 = file.read_u64()?;
        let create: i64 = file.read_i64()?;
        let headeroffset: u64 = file.read_u64()?;
        let headersize: u64 = file.read_u64()?;
        let headerspace: u64 = file.read_u64()?;

        info!("Reading Graph header");

        let header = file.read_header(headeroffset, headersize)?;

        let d = Disk {
            diskname,
            disksize,
            file,
            version,
            create,
            headeroffset,
            headersize,
            headerspace,
            header,
        };

        Ok(d)
    }

    pub fn get_name(&self) -> &String {
        &self.diskname
    }
}

pub trait IOUTILS {
    fn read_u64(&mut self) -> io::Result<u64>;
    fn read_i64(&mut self) -> io::Result<i64>;
    fn read_header(&mut self, offset: u64, size: u64) -> io::Result<Header>;

    fn write_u64(&mut self, val: u64) -> io::Result<()>;
    fn write_i64(&mut self, val: i64) -> io::Result<()>;
    fn write_string(&mut self, buffer: String) -> io::Result<()>;
}

// Specific helper functions for handling raw IO with the DOVE storage
impl IOUTILS for File {
    fn read_u64(&mut self) -> io::Result<u64> {
        let mut buf: [u8; 8] = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(bytes_to_u64(&buf))
    }

    fn read_i64(&mut self) -> io::Result<i64> {
        let mut buf: [u8; 8] = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(bytes_to_i64(&buf))
    }

    fn read_header(&mut self, offset: u64, size: u64) -> io::Result<Header> {
        self.seek(SeekFrom::Start(offset))?;
        let mut buffer = Vec::with_capacity(size as usize);
        {
            let f = Read::by_ref(self);
            f.take(size).read_to_end(&mut buffer)?;
        }

        let s = match str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Failed to parse string as UTF8",
                ))
            }
        };
        let header: Header = serde_json::from_str(s)?;
        Ok(header)
    }

    fn write_u64(&mut self, val: u64) -> io::Result<()> {
        self.write_all(&u64_to_bytes(val))
    }

    fn write_i64(&mut self, val: i64) -> io::Result<()> {
        self.write_all(&i64_to_bytes(val))
    }

    fn write_string(&mut self, buffer: String) -> io::Result<()> {
        self.write_all(buffer.as_bytes())
    }
}

#[cfg(target_endian = "big")]
pub fn bytes_to_i64(x: &[u8; 8]) -> i64 {
    ((x[0] as i64) << 56) + ((x[1] as i64) << 48) + ((x[2] as i64) << 40) + ((x[3] as i64) << 32)
        + ((x[4] as i64) << 24) + ((x[5] as i64) << 16) + ((x[6] as i64) << 8)
        + ((x[7] as i64) << 0)
}

#[cfg(target_endian = "little")]
pub fn bytes_to_i64(x: &[u8; 8]) -> i64 {
    ((x[7] as i64) << 56) + ((x[6] as i64) << 48) + ((x[5] as i64) << 40) + ((x[4] as i64) << 32)
        + ((x[3] as i64) << 24) + ((x[2] as i64) << 16) + ((x[1] as i64) << 8)
        + ((x[0] as i64) << 0)
}

#[cfg(target_endian = "big")]
pub fn i64_to_bytes(x: i64) -> [u8; 8] {
    [
        ((x >> 56) & 0xff) as u8,
        ((x >> 48) & 0xff) as u8,
        ((x >> 40) & 0xff) as u8,
        ((x >> 32) & 0xff) as u8,
        ((x >> 24) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 8) & 0xff) as u8,
        ((x >> 0) & 0xff) as u8,
    ]
}

#[cfg(target_endian = "little")]
pub fn i64_to_bytes(x: i64) -> [u8; 8] {
    [
        ((x >> 0) & 0xff) as u8,
        ((x >> 8) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 24) & 0xff) as u8,
        ((x >> 32) & 0xff) as u8,
        ((x >> 40) & 0xff) as u8,
        ((x >> 48) & 0xff) as u8,
        ((x >> 56) & 0xff) as u8,
    ]
}

#[cfg(target_endian = "big")]
pub fn bytes_to_u64(x: &[u8; 8]) -> u64 {
    ((x[0] as u64) << 56) + ((x[1] as u64) << 48) + ((x[2] as u64) << 40) + ((x[3] as u64) << 32)
        + ((x[4] as u64) << 24) + ((x[5] as u64) << 16) + ((x[6] as u64) << 8)
        + ((x[7] as u64) << 0)
}

#[cfg(target_endian = "little")]
pub fn bytes_to_u64(x: &[u8; 8]) -> u64 {
    ((x[7] as u64) << 56) + ((x[6] as u64) << 48) + ((x[5] as u64) << 40) + ((x[4] as u64) << 32)
        + ((x[3] as u64) << 24) + ((x[2] as u64) << 16) + ((x[1] as u64) << 8)
        + ((x[0] as u64) << 0)
}

#[cfg(target_endian = "big")]
pub fn u64_to_bytes(x: u64) -> [u8; 8] {
    [
        ((x >> 56) & 0xff) as u8,
        ((x >> 48) & 0xff) as u8,
        ((x >> 40) & 0xff) as u8,
        ((x >> 32) & 0xff) as u8,
        ((x >> 24) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 8) & 0xff) as u8,
        ((x >> 0) & 0xff) as u8,
    ]
}

#[cfg(target_endian = "little")]
pub fn u64_to_bytes(x: u64) -> [u8; 8] {
    [
        ((x >> 0) & 0xff) as u8,
        ((x >> 8) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 24) & 0xff) as u8,
        ((x >> 32) & 0xff) as u8,
        ((x >> 40) & 0xff) as u8,
        ((x >> 48) & 0xff) as u8,
        ((x >> 56) & 0xff) as u8,
    ]
}
