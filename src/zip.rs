use std::fs::File;
use zip::ZipArchive;
use crate::PresentationError;

///
/// 打开ZIP文件
///
pub fn open(path: &str) -> Result<ZipArchive<File>, PresentationError> {
    let file = File::open(path);
    match file {
        Ok(file) => {
            Ok(ZipArchive::new(file).unwrap())
        }
        Err(e) => {
            Err(PresentationError::ReadError(format!("{}", e)))
        }
    }
}



