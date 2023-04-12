use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtColor;

use crate::schemas::presentation::main::CtTlCommonBehaviorData;

use crate::schemas::presentation::main::CtTlByAnimateColorTransform;

/**
 * @author : zhilong.zhou
 * @description : CT_TLAnimateColorBehavior
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlAnimateColorBehavior {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@clrSpc")]
    pub clr_spc_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dir")]
    pub dir_attr: Option<String>,

    #[serde(rename(serialize = "p:cBhvr", deserialize = "cBhvr"))]
    pub c_bhvr: CtTlCommonBehaviorData,

    #[serde(rename(serialize = "p:by", deserialize = "by"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<CtTlByAnimateColorTransform>,

    #[serde(rename(serialize = "p:from", deserialize = "from"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<CtColor>,

    #[serde(rename(serialize = "p:to", deserialize = "to"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<CtColor>,
}
