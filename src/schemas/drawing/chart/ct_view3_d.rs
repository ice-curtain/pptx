use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtRotX;

use crate::schemas::drawing::chart::CtDepthPercent;

use crate::schemas::drawing::chart::CtRotY;

use crate::schemas::drawing::chart::CtPerspective;

use crate::schemas::drawing::chart::CtHPercent;

/**
 * @author : zhilong.zhou
 * @description : CT_View3D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtView3D {
    #[serde(rename(serialize = "rotX", deserialize = "rotX"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rot_x: Option<CtRotX>,

    #[serde(rename(serialize = "hPercent", deserialize = "hPercent"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_percent: Option<CtHPercent>,

    #[serde(rename(serialize = "rotY", deserialize = "rotY"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rot_y: Option<CtRotY>,

    #[serde(rename(serialize = "depthPercent", deserialize = "depthPercent"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_percent: Option<CtDepthPercent>,

    #[serde(rename(serialize = "rAngAx", deserialize = "rAngAx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_ang_ax: Option<CtBoolean>,

    #[serde(rename(serialize = "perspective", deserialize = "perspective"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perspective: Option<CtPerspective>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
