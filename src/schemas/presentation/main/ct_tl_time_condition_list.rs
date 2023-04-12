use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlTimeCondition;

/**
 * @author : zhilong.zhou
 * @description : CT_TLTimeConditionList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTimeConditionList {
    #[serde(rename(serialize = "p:cond", deserialize = "cond"))]
    pub cond: Vec<CtTlTimeCondition>,
}
