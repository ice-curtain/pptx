use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLTriggerTimeNodeID
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTriggerTimeNodeId {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
