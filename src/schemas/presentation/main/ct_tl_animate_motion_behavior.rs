use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlCommonBehaviorData;

use crate::schemas::presentation::main::CtTlPoint;

/**
 * @author : zhilong.zhou
 * @description : CT_TLAnimateMotionBehavior
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlAnimateMotionBehavior {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@origin")]
    pub origin_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@path")]
    pub path_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@pathEditMode")]
    pub path_edit_mode_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rAng")]
    pub r_ang_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@ptsTypes")]
    pub pts_types_attr: Option<String>,

    #[serde(rename(serialize = "p:cBhvr", deserialize = "cBhvr"))]
    pub c_bhvr: CtTlCommonBehaviorData,

    #[serde(rename(serialize = "p:by", deserialize = "by"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<CtTlPoint>,

    #[serde(rename(serialize = "p:from", deserialize = "from"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<CtTlPoint>,

    #[serde(rename(serialize = "p:to", deserialize = "to"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<CtTlPoint>,

    #[serde(rename(serialize = "p:rCtr", deserialize = "rCtr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_ctr: Option<CtTlPoint>,
}
