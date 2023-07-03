use serde::{Serialize, Deserialize};

pub const MEDIA_DIR: &str = "ppt/media";


pub type UnSupportPart = BinaryPart;
pub type MediaPart = UnSupportPart;


/// no field,just placeholder
#[derive(Serialize, Deserialize, Debug)]
pub struct BinaryPart {}


