use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

use crate::schemas::drawing::main::CtNonVisualGraphicFrameProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_GvmlGraphicFrameNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGvmlGraphicFrameNonVisual {
    #[serde(rename(serialize = "a:cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "a:cNvGraphicFramePr", deserialize = "cNvGraphicFramePr"))]
    pub c_nv_graphic_frame_pr: CtNonVisualGraphicFrameProperties,
}
