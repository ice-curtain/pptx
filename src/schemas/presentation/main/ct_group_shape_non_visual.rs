use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualGroupDrawingShapeProps;

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

use crate::schemas::presentation::main::CtApplicationNonVisualDrawingProps;

/**
 * @author : zhilong.zhou
 * @description : CT_GroupShapeNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGroupShapeNonVisual {
    #[serde(rename(serialize = "p:cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "p:cNvGrpSpPr", deserialize = "cNvGrpSpPr"))]
    pub c_nv_grp_sp_pr: CtNonVisualGroupDrawingShapeProps,

    #[serde(rename(serialize = "p:nvPr", deserialize = "nvPr"))]
    pub nv_pr: CtApplicationNonVisualDrawingProps,
}
