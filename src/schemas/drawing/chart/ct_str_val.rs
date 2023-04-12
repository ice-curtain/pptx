use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_StrVal
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtStrVal {
    #[serde(rename = "@idx")]
    pub idx_attr: String,

    #[serde(rename(serialize = "v", deserialize = "v"))]
    pub v: String,
}
