use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtSphereCoords;

/**
 * @author : zhilong.zhou
 * @description : CT_Camera
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCamera {
    #[serde(rename = "@prst")]
    pub prst_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fov")]
    pub fov_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@zoom")]
    pub zoom_attr: Option<String>,

    #[serde(rename(serialize = "a:rot", deserialize = "rot"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rot: Option<CtSphereCoords>,
}
