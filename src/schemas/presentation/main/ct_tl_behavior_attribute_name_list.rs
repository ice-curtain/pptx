use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLBehaviorAttributeNameList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlBehaviorAttributeNameList {
    #[serde(rename(serialize = "p:attrName", deserialize = "attrName"))]
    pub attr_name: Vec<String>,
}
