use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_PictureStackUnit
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPictureStackUnit {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
