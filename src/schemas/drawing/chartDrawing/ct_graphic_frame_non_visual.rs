use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

use crate::schemas::drawing::main::CtNonVisualGraphicFrameProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_GraphicFrameNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGraphicFrameNonVisual {
    #[serde(rename(serialize = "cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "cNvGraphicFramePr", deserialize = "cNvGraphicFramePr"))]
    pub c_nv_graphic_frame_pr: CtNonVisualGraphicFrameProperties,
}
