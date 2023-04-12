use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlCommonTimeNodeData;

use crate::schemas::presentation::main::CtTlTimeConditionList;

/**
 * @author : zhilong.zhou
 * @description : CT_TLTimeNodeSequence
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTimeNodeSequence {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@concurrent")]
    pub concurrent_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@prevAc")]
    pub prev_ac_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@nextAc")]
    pub next_ac_attr: Option<String>,

    #[serde(rename(serialize = "p:cTn", deserialize = "cTn"))]
    pub c_tn: CtTlCommonTimeNodeData,

    #[serde(rename(serialize = "p:prevCondLst", deserialize = "prevCondLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_cond_lst: Option<CtTlTimeConditionList>,

    #[serde(rename(serialize = "p:nextCondLst", deserialize = "nextCondLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cond_lst: Option<CtTlTimeConditionList>,
}
