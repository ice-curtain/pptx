use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_PictureFormat
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPictureFormat {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
