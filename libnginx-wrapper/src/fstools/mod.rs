use super::http_server::NginxObj;
use std::{
    fs::{read_dir, OpenOptions},
    io::{BufReader, Read, BufWriter, Write},
};

pub mod read_ops;
pub mod write_ops;
