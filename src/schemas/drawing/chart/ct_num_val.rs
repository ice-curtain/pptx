use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_NumVal
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNumVal {
    #[serde(rename = "@idx")]
    pub idx_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@formatCode")]
    pub format_code_attr: Option<String>,

    #[serde(rename(serialize = "v", deserialize = "v"))]
    pub v: String,
}
