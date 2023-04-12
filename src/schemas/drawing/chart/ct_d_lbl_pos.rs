use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_DLblPos
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDLblPos {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
