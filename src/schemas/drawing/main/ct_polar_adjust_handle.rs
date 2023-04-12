use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAdjPoint2D;

/**
 * @author : zhilong.zhou
 * @description : CT_PolarAdjustHandle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPolarAdjustHandle {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@gdRefR")]
    pub gd_ref_r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@minR")]
    pub min_r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@maxR")]
    pub max_r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@gdRefAng")]
    pub gd_ref_ang_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@minAng")]
    pub min_ang_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@maxAng")]
    pub max_ang_attr: Option<String>,

    #[serde(rename(serialize = "a:pos", deserialize = "pos"))]
    pub pos: CtAdjPoint2D,
}
