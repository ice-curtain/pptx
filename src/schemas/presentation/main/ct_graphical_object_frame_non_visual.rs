use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

use crate::schemas::presentation::main::CtApplicationNonVisualDrawingProps;

use crate::schemas::drawing::main::CtNonVisualGraphicFrameProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_GraphicalObjectFrameNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGraphicalObjectFrameNonVisual {
    #[serde(rename(serialize = "p:cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "p:cNvGraphicFramePr", deserialize = "cNvGraphicFramePr"))]
    pub c_nv_graphic_frame_pr: CtNonVisualGraphicFrameProperties,

    #[serde(rename(serialize = "p:nvPr", deserialize = "nvPr"))]
    pub nv_pr: CtApplicationNonVisualDrawingProps,
}
