use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualConnectorProperties;

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

/**
 * @author : zhilong.zhou
 * @description : CT_ConnectorNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtConnectorNonVisual {
    #[serde(rename(serialize = "cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "cNvCxnSpPr", deserialize = "cNvCxnSpPr"))]
    pub c_nv_cxn_sp_pr: CtNonVisualConnectorProperties,
}
