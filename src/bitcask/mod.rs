use std::io::{BufReader, BufWriter, Read, Write};
#[allow(unused_imports)]
use std::{
    
    fs::File,
    fs::OpenOptions,
    fs::create_dir_all,
    
    io::Seek,
    io::SeekFrom,
    io::Result,


};
use camino::Utf8PathBuf;


#[allow(dead_code)]
const KEY_VAL_HEADER_LEN: u32 = 4;


pub fn create_or_open(path: &Utf8PathBuf) -> Result<File>  {
    let parent = path.parent();
    let mut options = OpenOptions::new();

    if let Some(dir) = parent {
        if ! dir.exists() {
            create_dir_all(dir)?;
        }
    }

    options.read(true).write(true).append(true);
    if ! path.exists() {
        options.create(true);
    }
    let f = options.open(path);
    f
}


#[allow(dead_code)]
pub struct Log {
    path: Utf8PathBuf,
    f: File
}


#[allow(dead_code, unused_variables)]
impl Log {
    fn new(path: Utf8PathBuf) -> Self {
        let f = create_or_open(&path).unwrap();
        Self {path, f}
    }

    fn write(&mut self, key: &[u8], value: Option<&[u8]>) ->Result<(u64, u32)>  {
        let key_len = key.len() as u32;
        let value_len = value.map_or(0, |v| v.len() as u32);
        let len = KEY_VAL_HEADER_LEN * 2 + key_len + value_len;
        let mut w = BufWriter::with_capacity(len as usize, &mut self.f);

        let mut pos = w.seek(SeekFrom::End(0))?;
        pos += (KEY_VAL_HEADER_LEN * 2) as u64 + key_len as u64;

        w.write_all(&key_len.to_be_bytes())?;
        w.write_all(&value_len.to_be_bytes())?;
        w.write_all(&key)?;
        if let Some(v) = value {
            w.write_all(v)?;

        }
        w.flush()?;

        Ok((pos, value_len))
    }

    fn read(&mut self, pos: u64, len: u32) -> Result<Vec<u8>> {
        let f = &mut self.f;
        let mut v = vec![0_u8; len as usize];
        f.seek(SeekFrom::Start(pos))?;
        f.read(&mut v)?;
        Ok(v)
    }
}