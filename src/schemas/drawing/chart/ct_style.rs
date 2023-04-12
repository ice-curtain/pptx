use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Style
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtStyle {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
