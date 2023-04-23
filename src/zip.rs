use std::fs::File;
use zip::ZipArchive;

///
/// 打开ZIP文件
///
pub fn open(path: &str) -> ZipArchive<File> {
    let file = File::open(path);
    match file{
        Ok(file)=>{
            let zip: ZipArchive<File> = ZipArchive::new(file).unwrap();
            return zip;
        }
        Err(e)=>{
            panic!("read file error,{}",e);
        }
    }
}



