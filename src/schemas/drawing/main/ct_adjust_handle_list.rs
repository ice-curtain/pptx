use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtPolarAdjustHandle;

use crate::schemas::drawing::main::CtXyAdjustHandle;

/**
 * @author : zhilong.zhou
 * @description : CT_AdjustHandleList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAdjustHandleList {
    #[serde(rename(serialize = "a:ahXY", deserialize = "ahXY"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ah_xy: Option<Vec<CtXyAdjustHandle>>,

    #[serde(rename(serialize = "a:ahPolar", deserialize = "ahPolar"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ah_polar: Option<Vec<CtPolarAdjustHandle>>,
}
