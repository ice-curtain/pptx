use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtSphereCoords;

/**
 * @author : zhilong.zhou
 * @description : CT_LightRig
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLightRig {
    #[serde(rename = "@rig")]
    pub rig_attr: String,

    #[serde(rename = "@dir")]
    pub dir_attr: String,

    #[serde(rename(serialize = "a:rot", deserialize = "rot"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rot: Option<CtSphereCoords>,
}
