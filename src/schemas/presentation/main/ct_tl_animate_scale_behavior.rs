use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlCommonBehaviorData;

use crate::schemas::presentation::main::CtTlPoint;

/**
 * @author : zhilong.zhou
 * @description : CT_TLAnimateScaleBehavior
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlAnimateScaleBehavior {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@zoomContents")]
    pub zoom_contents_attr: Option<String>,

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
}
