use std::fs::File;
use zip::ZipArchive;

///
/// 打开ZIP文件
///
pub fn open(path: &str) -> ZipArchive<File> {
    let file: File = File::open(path).unwrap();
    let zip: ZipArchive<File> = ZipArchive::new(file).unwrap();
    return zip;
}



