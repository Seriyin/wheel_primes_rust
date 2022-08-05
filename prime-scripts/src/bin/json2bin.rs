use bincode::Options;
use regex::Regex;
use std::fs::{DirEntry, File};
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() -> Result<(), Error> {
    let dir_string = env::args()
        .nth(1)
        .ok_or_else(|| Error::new(ErrorKind::Other, "No argument path supplied"))?;

    let dir_path = Path::new(dir_string.as_str());

    let r_match = Regex::new(r"primes\d+(E\d+)?.json")
        .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))?;
    for entry in fs::read_dir(dir_path)? {
        match entry {
            Ok(e) => {
                let meta = e.metadata().map(|meta| meta.file_type().is_file())?;
                if meta {
                    process_file(&e, &r_match).unwrap_or_else(|err| {
                        println!("Skipping {:?} due to {:?}", &e.file_name(), err)
                    })
                }
            }
            Err(err) => println!("Directory can't be read: {}", err),
        };
    }

    return Ok(());
}

pub fn process_file(e: &DirEntry, reg: &Regex) -> Result<(), Error> {
    let name = e.file_name().into_string().map_err(|os| {
        Error::new(
            ErrorKind::Other,
            format!("OsString not valid UTF-8: {:?}", os),
        )
    })?;

    if reg.is_match(name.as_str()) {
        serialize_from_path(e.path())
    } else {
        Ok(())
    }
}

pub fn serialize_from_path(p: PathBuf) -> Result<(), Error> {
    let mut canonical = p.canonicalize()?;
    let read_path = canonical.as_path();
    let reader = BufReader::new(File::open(read_path)?);
    return if canonical.set_extension("bin") {
        let write_path = canonical.as_path();
        let writer = BufWriter::new(File::create(write_path)?);

        let primes_vec: Vec<usize> = serde_json::from_reader(reader)?;
        bincode::options()
            .serialize_into(writer, &primes_vec)
            .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("Couldn't set extension: {:?}", canonical.as_os_str()),
        ))
    };
}
