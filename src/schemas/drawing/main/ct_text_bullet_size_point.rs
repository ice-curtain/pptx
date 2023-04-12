use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TextBulletSizePoint
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextBulletSizePoint {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
