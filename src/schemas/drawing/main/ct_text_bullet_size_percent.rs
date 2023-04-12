use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TextBulletSizePercent
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextBulletSizePercent {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
