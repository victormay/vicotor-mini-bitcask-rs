use std::fmt;
#[allow(unused_imports)]
use std::{
    str,

    path::Path,
    path::PathBuf,

    env::current_dir,

    fs::File,
    fs::OpenOptions,
    fs::create_dir_all,

    io::BufReader,
    io::BufWriter,
    io::ErrorKind,
    io::Result,
    io::Read,
    io::Write,
    io::Seek,
    io::SeekFrom
};

#[allow(unused_imports)]
use camino::{
    Utf8Path,
    Utf8PathBuf
};


#[allow(dead_code)]
pub fn demo1() {
    let path = Path::new("/tmp/foo/bar.txt");
    println!("{path:?}");
    println!("{:?}", path.parent());
    println!("{:?}", path.file_stem());
    println!("{:?}", path.extension());
}


#[allow(dead_code)]
pub fn demo2() {
    let mut path = PathBuf::from("c:\\");
    path.push("users");
    path.push("ai");
    path.push("demo");
    path.set_extension("dll");
    println!("{path:?}");
    println!("{:?}", path.extension());

    let path2: PathBuf = ["c:\\", "Users", "Administrator", "Downloads", "path_demo"].iter().collect();
    println!("{path2:?}");

    let curr = current_dir().unwrap();
    println!("{curr:?}");
}


#[allow(dead_code)]
pub fn create_and_open(file_path: &Utf8PathBuf) -> Result<File> {
    // 获取文件目录
    let parent = file_path.parent();
    // 创建文件夹
    if let Some(dir) = parent {
        if ! dir.exists() {
            create_dir_all(dir)?;
        }
    }
    // 创建options
    let mut options = OpenOptions::new();
    // 是否创建文件
    if !file_path.exists() {
        options.create(true);
    }
    // 读和追加写
    options.read(true).write(true).append(true);
    // 返回文件
    let f = options.open(file_path)?;
    Ok(f)
}


#[allow(dead_code)]
pub fn demo3() {
    let root = Utf8PathBuf::from("./path_demo/demo1");
    let file_path = root.join("demo.txt");
    println!("{root:?}");
    println!("{file_path:?}");

    let mut f = create_and_open(&file_path).unwrap();
    println!("{f:?}");
    let mut msg = String::new();
    let res = f.read_to_string(&mut msg);
    println!("read: {res:?}");
    let res = f.write("hello world\n".as_bytes());
    println!("write: {res:?}");
    println!("{msg}");
}


#[allow(dead_code)]
struct Log {
    path: Utf8PathBuf,
    f: File
}


#[allow(dead_code)]
impl Log {
    fn new(path: Utf8PathBuf) -> Self {
        let f = create_and_open(&path).unwrap();
        Self { path, f }
    }

    fn read(&mut self) -> Result<String> {
        let mut msg = String::new();
        let f = &mut self.f;
        f.read_to_string(&mut msg)?;
        Ok(msg)
    }

    fn write<'a>(&mut self, msg: &'a str) -> Result<&'a str> {
        let f = &mut self.f;
        f.write(msg.as_bytes())?;
        self.f.flush()?;
        Ok(msg)
    }

    fn write_(&mut self, key: &[u8], value: Option<&[u8]>) -> Result<u32> {
        let key_len = key.len() as u32;
        let value_len = value.map_or(0, |v| v.len() as u32);
        let len = 8 + key_len + value_len;
        let mut bf_w = BufWriter::with_capacity(len as usize, &mut self.f);
        bf_w.write_all(&key_len.to_be_bytes())?;
        bf_w.write_all(&value_len.to_be_bytes())?;
        bf_w.write_all(key)?;
        if let Some(v) = value {
            bf_w.write_all(v)?;
        }
        Ok(len)
    }

    fn test(&mut self) -> Result<(String, String)>{
        let mut bu_r = BufReader::with_capacity(8, &mut self.f);
        let mut buffer = [0_u8; 4];
        bu_r.read(&mut buffer)?;
        let key_len = u32::from_be_bytes(buffer) as usize;
        bu_r.read(&mut buffer)?;
        let value_len = u32::from_be_bytes(buffer)as usize;
        let mut key_bytes: Vec<u8> = vec![0; key_len];
        let mut value_bytes: Vec<u8> = vec![0; value_len];
        bu_r.read(&mut key_bytes)?;
        let key = String::from_utf8(key_bytes.clone()).unwrap();
        bu_r.read(&mut value_bytes)?;
        let value = String::from_utf8(value_bytes.clone()).unwrap();
        Ok((key, value))
    }

    fn test2(&mut self) -> Result<()> {
        let file_len = self.f.metadata()?.len();
        let mut bf_r = BufReader::new(&mut self.f);
        let mut pos = bf_r.seek(SeekFrom::Start(0))?;
        println!("{pos:?}");

        while pos < file_len {
            let mut head: Vec<u8> = vec![0_u8; 8];

            bf_r.read(&mut head)?;
            pos += head.len() as u64;
            // println!("{pos:?}");
            
            let key_len = u32::from_be_bytes(<[u8; 4]>::try_from(&head[..4]).unwrap()) as usize;
            let value_len = u32::from_be_bytes(<[u8; 4]>::try_from(&head[4..]).unwrap()) as usize;

            let mut key_bytes = vec![0_u8; key_len];
            let mut value_bytes = vec![0_u8; value_len];
            
            bf_r.read(&mut key_bytes)?;
            pos += key_bytes.len() as u64;
            // println!("{pos:?}");
            bf_r.read(&mut value_bytes)?;
            pos += value_bytes.len() as u64;
            // println!("{pos:?}");
            
            let key = String::from_utf8(key_bytes).unwrap();
            let value = String::from_utf8(value_bytes).unwrap();

            println!("{key}: {value}, seek: {pos}, len: {}", head.len() + value_len + key_len);
        }
        
        Ok(())
    }

}


#[allow(dead_code)]
pub fn demo4() {
    let path = Utf8PathBuf::from("./path_demo/demo1/demo.db");
    let mut log = Log::new(path);
    
    // for i in 1..=5 {
    //     let key = format!("key_{i}");
    //     let key = key.as_bytes();
    //     let value = format!("{i}");
    //     let value = value.as_bytes();
    //     let res = log.write_(key, Some(value));
    //     println!("{res:?}");
    // }
    
    println!("{:?}", log.test2());
}