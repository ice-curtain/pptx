use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualDrawingShapeProps;

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

/**
 * @author : zhilong.zhou
 * @description : CT_ShapeNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtShapeNonVisual {
    #[serde(rename(serialize = "cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "cNvSpPr", deserialize = "cNvSpPr"))]
    pub c_nv_sp_pr: CtNonVisualDrawingShapeProps,
}
