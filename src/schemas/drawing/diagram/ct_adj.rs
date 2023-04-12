use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Adj
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAdj {
    #[serde(rename = "@idx")]
    pub idx_attr: String,

    #[serde(rename = "@val")]
    pub val_attr: String,
}
