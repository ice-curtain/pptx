use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_StringTag
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtStringTag {
    #[serde(rename = "@name")]
    pub name_attr: String,

    #[serde(rename = "@val")]
    pub val_attr: String,
}
