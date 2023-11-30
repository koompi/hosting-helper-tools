use super::http_server::{
    nginx_features::NginxFeatures, nginx_obj::NginxObj, target_site::TargetSite,
};
use std::{
    fs::{read_dir, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
};

pub mod read_ops;
pub(crate) mod write_ops;

pub fn write_file(
    destination_file: &str,
    config: &str,
    is_continuing: bool,
) -> Result<(), (u16, String)> {
    write_ops::write_file(destination_file, config, is_continuing)
}

pub fn read_file<S: AsRef<str>>(source_file: S) -> String {
    read_ops::read_file(source_file.as_ref())
}