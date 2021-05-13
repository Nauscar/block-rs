use bincode::{serialize_into, deserialize_from};
use serde::{Deserialize, Serialize};
use log::debug;
use std::env::{current_dir, current_exe};
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, ErrorKind, Write};
use std::path::PathBuf;

use crate::error::{BoxResult, NoneError};

pub trait Storage = Default + for<'de> Deserialize<'de> + Serialize; 

pub struct Block<T> 
where
    T: Storage,
{
    #[cfg(Debug)]
    _key: String,
    value: T,
    path: PathBuf,
}

fn get_path(key: &str) -> BoxResult<PathBuf> {
    let mut path_buf = current_dir()?.join(current_exe()?.file_stem().ok_or(NoneError)?);
    create_dir_all(&path_buf)?;
    path_buf.push(key);

    Ok(path_buf)
}

fn pre_load<'de, T>(path: &PathBuf) -> BoxResult<T>
where
    T: Storage,
{
    let result = File::open(path);
    let file = match result {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                return Ok(T::default())
            } else {
                return Err(Box::new(error))
            }
        }
    };

    let reader = BufReader::new(file);
    let value: T = deserialize_from(reader)?;

    Ok(value)
}

impl<T> Block<T> 
where
    T: Storage,
{
    pub fn load(key: &str) -> Self {
        let path = get_path(key).unwrap().to_owned();
        let value: T = pre_load(&path).unwrap();
        debug!(" load: {}", path.display());

        cfg_if::cfg_if! {
            if #[cfg(Debug)] {
                return Self { _key: key.to_owned(), value, path }
            } else {
                return Self { value, path }
            }
        }
    }

    pub fn store(&self) -> BoxResult<&T> {
        debug!("store: {}", self.path.display());
        let mut writer = BufWriter::new(File::create(&self.path)?);
        serialize_into(&mut writer, &self.value)?;
        writer.flush()?;

        Ok(&self.value)
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }
}

impl<T> Drop for Block<T>
where
    T: Storage,
{
    fn drop(&mut self) {
        let _result = self.store();

        #[cfg(Debug)]
        _result.unwrap();
    }
}