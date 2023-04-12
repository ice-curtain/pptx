use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLTriggerRuntimeNode
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTriggerRuntimeNode {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
