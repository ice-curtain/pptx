use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualGroupDrawingShapeProps;

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

/**
 * @author : zhilong.zhou
 * @description : CT_GvmlGroupShapeNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGvmlGroupShapeNonVisual {
    #[serde(rename(serialize = "a:cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "a:cNvGrpSpPr", deserialize = "cNvGrpSpPr"))]
    pub c_nv_grp_sp_pr: CtNonVisualGroupDrawingShapeProps,
}
