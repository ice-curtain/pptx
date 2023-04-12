use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlAnimVariant;

use crate::schemas::presentation::main::CtTlCommonBehaviorData;

/**
 * @author : zhilong.zhou
 * @description : CT_TLAnimateEffectBehavior
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlAnimateEffectBehavior {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@transition")]
    pub transition_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@filter")]
    pub filter_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@prLst")]
    pub pr_lst_attr: Option<String>,

    #[serde(rename(serialize = "p:cBhvr", deserialize = "cBhvr"))]
    pub c_bhvr: CtTlCommonBehaviorData,

    #[serde(rename(serialize = "p:progress", deserialize = "progress"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<CtTlAnimVariant>,
}
