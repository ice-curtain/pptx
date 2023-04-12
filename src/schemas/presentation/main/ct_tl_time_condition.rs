use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlTriggerTimeNodeId;

use crate::schemas::presentation::main::CtTlTriggerRuntimeNode;

use crate::schemas::presentation::main::CtTlTimeTargetElement;

/**
 * @author : zhilong.zhou
 * @description : CT_TLTimeCondition
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTimeCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@evt")]
    pub evt_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@delay")]
    pub delay_attr: Option<String>,

    #[serde(rename(serialize = "p:tgtEl", deserialize = "tgtEl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgt_el: Option<CtTlTimeTargetElement>,

    #[serde(rename(serialize = "p:tn", deserialize = "tn"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tn: Option<CtTlTriggerTimeNodeId>,

    #[serde(rename(serialize = "p:rtn", deserialize = "rtn"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtn: Option<CtTlTriggerRuntimeNode>,
}
