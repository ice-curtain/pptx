use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlCommonBehaviorData;

/**
 * @author : zhilong.zhou
 * @description : CT_TLCommandBehavior
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlCommandBehavior {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@cmd")]
    pub cmd_attr: Option<String>,

    #[serde(rename(serialize = "p:cBhvr", deserialize = "cBhvr"))]
    pub c_bhvr: CtTlCommonBehaviorData,
}
