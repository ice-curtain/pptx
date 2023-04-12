use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlCommonBehaviorData;

/**
 * @author : zhilong.zhou
 * @description : CT_TLAnimateRotationBehavior
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlAnimateRotationBehavior {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@by")]
    pub by_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@from")]
    pub from_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@to")]
    pub to_attr: Option<String>,

    #[serde(rename(serialize = "p:cBhvr", deserialize = "cBhvr"))]
    pub c_bhvr: CtTlCommonBehaviorData,
}
