use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtGraphicalObject;

use crate::schemas::presentation::main::CtGraphicalObjectFrameNonVisual;

use crate::schemas::presentation::main::CtExtensionListModify;

use crate::schemas::drawing::main::CtTransform2D;

/**
 * @author : zhilong.zhou
 * @description : CT_GraphicalObjectFrame
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGraphicalObjectFrame {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bwMode")]
    pub bw_mode_attr: Option<String>,

    #[serde(rename(serialize = "p:nvGraphicFramePr", deserialize = "nvGraphicFramePr"))]
    pub nv_graphic_frame_pr: CtGraphicalObjectFrameNonVisual,

    #[serde(rename(serialize = "p:xfrm", deserialize = "xfrm"))]
    pub xfrm: CtTransform2D,

    #[serde(rename(serialize = "p:graphic", deserialize = "graphic"))]
    pub graphic: CtGraphicalObject,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,
}
