use super::{BufWriter, OpenOptions, Write};

pub(crate) fn write_file(
    destination_file: &str,
    config: &str,
    is_continuing: bool,
) -> Result<(), (u16, String)> {
    let open_file = match is_continuing {
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
    };

    let open_file = match open_file {
        Ok(open_file) => Ok(open_file),
        Err(err) => Err((500, err.to_string() + "\nPlease use SUDO")),
    }?;

    BufWriter::new(open_file)
        .write_all(config.as_bytes())
        .unwrap();

    Ok(())
}

pub(crate) fn write_bin_file(
    destination_file: &str,
    config: &[u8],
    is_continuing: bool,
) -> Result<(), (u16, String)> {
    let open_file = match is_continuing {
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
    };

    let open_file = match open_file {
        Ok(open_file) => Ok(open_file),
        Err(err) => Err((500, err.to_string() + "\nPlease use SUDO")),
    }?;

    BufWriter::new(open_file)
        .write_all(config)
        .unwrap();

    Ok(())
}