
use serde::{Serialize,Deserialize};

pub const MEDIA_DIR: &str = "ppt/media";

#[derive(Serialize,Deserialize,Debug)]
pub struct Media{
   pub buf:Vec<u8>
}
