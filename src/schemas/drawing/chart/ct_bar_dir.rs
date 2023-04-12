use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_BarDir
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBarDir {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@val")]
    pub val_attr: Option<String>,
}
