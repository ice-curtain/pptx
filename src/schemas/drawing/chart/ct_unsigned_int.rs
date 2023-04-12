use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_UnsignedInt
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtUnsignedInt {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
