use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtGvmlGraphicFrameNonVisual;

use crate::schemas::drawing::main::CtGraphicalObject;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtTransform2D;

/**
 * @author : zhilong.zhou
 * @description : CT_GvmlGraphicalObjectFrame
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGvmlGraphicalObjectFrame {
    #[serde(rename(serialize = "a:nvGraphicFramePr", deserialize = "nvGraphicFramePr"))]
    pub nv_graphic_frame_pr: CtGvmlGraphicFrameNonVisual,

    #[serde(rename(serialize = "a:graphic", deserialize = "graphic"))]
    pub graphic: CtGraphicalObject,

    #[serde(rename(serialize = "a:xfrm", deserialize = "xfrm"))]
    pub xfrm: CtTransform2D,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
