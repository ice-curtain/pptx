use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_BuiltInUnit
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBuiltInUnit {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@val")]
    pub val_attr: Option<String>,
}