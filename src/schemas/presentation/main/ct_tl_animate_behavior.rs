use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlTimeAnimateValueList;

use crate::schemas::presentation::main::CtTlCommonBehaviorData;

/**
 * @author : zhilong.zhou
 * @description : CT_TLAnimateBehavior
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlAnimateBehavior {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@by")]
    pub by_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@from")]
    pub from_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@to")]
    pub to_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@calcmode")]
    pub calcmode_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@valueType")]
    pub value_type_attr: Option<String>,

    #[serde(rename(serialize = "p:cBhvr", deserialize = "cBhvr"))]
    pub c_bhvr: CtTlCommonBehaviorData,

    #[serde(rename(serialize = "p:tavLst", deserialize = "tavLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tav_lst: Option<CtTlTimeAnimateValueList>,
}
