use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualDrawingShapeProps;

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

/**
 * @author : zhilong.zhou
 * @description : CT_GvmlShapeNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGvmlShapeNonVisual {
    #[serde(rename(serialize = "a:cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "a:cNvSpPr", deserialize = "cNvSpPr"))]
    pub c_nv_sp_pr: CtNonVisualDrawingShapeProps,
}
