use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chartDrawing::CtGraphicFrameNonVisual;

use crate::schemas::drawing::main::CtGraphicalObject;

use crate::schemas::drawing::main::CtTransform2D;

/**
 * @author : zhilong.zhou
 * @description : CT_GraphicFrame
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGraphicFrame {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@macro")]
    pub r#macro_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fPublished")]
    pub f_published_attr: Option<String>,

    #[serde(rename(serialize = "nvGraphicFramePr", deserialize = "nvGraphicFramePr"))]
    pub nv_graphic_frame_pr: CtGraphicFrameNonVisual,

    #[serde(rename(serialize = "xfrm", deserialize = "xfrm"))]
    pub xfrm: CtTransform2D,

    #[serde(rename(serialize = "graphic", deserialize = "graphic"))]
    pub graphic: CtGraphicalObject,
}
