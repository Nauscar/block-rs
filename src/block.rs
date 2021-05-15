use bincode::{serialize_into, deserialize_from};
use global_counter::primitive::exact::CounterUsize;
use log::debug;
use serde::{Deserialize, Serialize};
use std::env::{current_dir, current_exe};
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, ErrorKind, Write};
use std::path::PathBuf;

use crate::error::{BoxResult, NoneError};

pub trait Storage = Default + for<'de> Deserialize<'de> + PartialEq + Serialize; 

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
    pub fn Log(value: T) {
        Block::default().set(value);
    }

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

    pub fn store(&self) -> BoxResult<bool> {
        let value = self.get().to_owned();
        if *value == T::default() {
            return Ok(false)
        }

        debug!("store: {}", self.path.display());
        let mut writer = BufWriter::new(File::create(&self.path)?);
        serialize_into(&mut writer, &self.value)?;
        writer.flush()?;

        Ok(true)
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }
}

impl<T> fmt::Debug for Block<T>
where
    T: fmt::Debug + Storage,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

static DEFAULT_COUNTER : CounterUsize = CounterUsize::new(0); 

impl<T> Default  for Block<T>
where T: Storage,
{
    fn default() -> Self {
        let count = DEFAULT_COUNTER.get();
        let key = &(stringify!(T).to_owned() + &count.to_string());
        DEFAULT_COUNTER.inc();
        let path = get_path(key).unwrap();
        debug!("default[{}]: {}", count.to_string(), path.display());

        cfg_if::cfg_if! {
            if #[cfg(Debug)] {
                return Self { _key: key.to_owned(), value: T::default(), path }
            } else {
                return Self { value: T::default(), path }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::{get_path, pre_load};
    use crate::block::Block;
    use crate::error::BoxResult;
    use crate::test::Test;
    use log::debug;

    #[test]
    fn test_load() {
        let block = Block::<Test>::load(stringify!(block));
        debug!("{:?}", block);
    }

    #[test]
    fn test_store() {
        Block::<Test>::default().store().unwrap();
    }

    #[test]
    fn test_get() {
        let test: Test = Block::<Test>::default().get().to_owned();
        assert_eq!(test, Test::default());
        debug!("{:?}", test);
    }

    #[test]
    fn test_set() {
        Block::<Test>::default().set(Test::new("set"));
    }

    #[test]
    fn test_log() {
        Block::<Test>::Log(Test::new("log"));
    }

    #[test]
    fn test_default() {
        Block::<Test>::default();
    }

    #[test]
    fn test_drop() -> BoxResult<()> {
        Ok(drop(Block::<Test>::default()))
    }
}