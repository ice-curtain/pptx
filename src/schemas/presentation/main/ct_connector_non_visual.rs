use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualConnectorProperties;

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

use crate::schemas::presentation::main::CtApplicationNonVisualDrawingProps;

/**
 * @author : zhilong.zhou
 * @description : CT_ConnectorNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtConnectorNonVisual {
    #[serde(rename(serialize = "p:cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "p:cNvCxnSpPr", deserialize = "cNvCxnSpPr"))]
    pub c_nv_cxn_sp_pr: CtNonVisualConnectorProperties,

    #[serde(rename(serialize = "p:nvPr", deserialize = "nvPr"))]
    pub nv_pr: CtApplicationNonVisualDrawingProps,
}
