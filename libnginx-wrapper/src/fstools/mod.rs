use super::http_server::NginxObj;
use std::{
    fs::{read_dir, OpenOptions},
    io::{BufReader, Read, BufWriter, Write},
};

pub(crate) mod read_ops;
pub(crate) mod write_ops;
