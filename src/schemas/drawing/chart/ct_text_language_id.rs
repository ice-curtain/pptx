use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TextLanguageID
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextLanguageId {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
