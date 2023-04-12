use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualGroupDrawingShapeProps;

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

/**
 * @author : zhilong.zhou
 * @description : CT_GroupShapeNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGroupShapeNonVisual {
    #[serde(rename(serialize = "cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "cNvGrpSpPr", deserialize = "cNvGrpSpPr"))]
    pub c_nv_grp_sp_pr: CtNonVisualGroupDrawingShapeProps,
}
