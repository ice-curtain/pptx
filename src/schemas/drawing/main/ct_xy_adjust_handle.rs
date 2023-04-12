use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAdjPoint2D;

/**
 * @author : zhilong.zhou
 * @description : CT_XYAdjustHandle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtXyAdjustHandle {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@gdRefX")]
    pub gd_ref_x_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@minX")]
    pub min_x_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@maxX")]
    pub max_x_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@gdRefY")]
    pub gd_ref_y_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@minY")]
    pub min_y_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@maxY")]
    pub max_y_attr: Option<String>,

    #[serde(rename(serialize = "a:pos", deserialize = "pos"))]
    pub pos: CtAdjPoint2D,
}
