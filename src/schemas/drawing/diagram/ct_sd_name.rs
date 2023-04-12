use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_SDName
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSdName {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lang")]
    pub lang_attr: Option<String>,

    #[serde(rename = "@val")]
    pub val_attr: String,
}
