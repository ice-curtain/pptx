use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlAnimVariant;

use crate::schemas::presentation::main::CtTlCommonBehaviorData;

/**
 * @author : zhilong.zhou
 * @description : CT_TLSetBehavior
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlSetBehavior {
    #[serde(rename(serialize = "p:cBhvr", deserialize = "cBhvr"))]
    pub c_bhvr: CtTlCommonBehaviorData,

    #[serde(rename(serialize = "p:to", deserialize = "to"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<CtTlAnimVariant>,
}
