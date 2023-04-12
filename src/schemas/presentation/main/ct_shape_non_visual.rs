use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualDrawingShapeProps;

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

use crate::schemas::presentation::main::CtApplicationNonVisualDrawingProps;

/**
 * @author : zhilong.zhou
 * @description : CT_ShapeNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtShapeNonVisual {
    #[serde(rename(serialize = "p:cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "p:cNvSpPr", deserialize = "cNvSpPr"))]
    pub c_nv_sp_pr: CtNonVisualDrawingShapeProps,

    #[serde(rename(serialize = "p:nvPr", deserialize = "nvPr"))]
    pub nv_pr: CtApplicationNonVisualDrawingProps,
}
