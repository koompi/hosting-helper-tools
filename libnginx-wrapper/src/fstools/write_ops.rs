use super::{BufWriter, OpenOptions, Write};

pub fn write_file(destination_file: &str, config: &str, is_continuing: bool) {
    BufWriter::new(
        match is_continuing {
            true => OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(destination_file),
            false => OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(destination_file),
        }
        .unwrap(),
    )
    .write_all(config.as_bytes())
    .unwrap();
}
