use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_LogBase
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLogBase {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
