use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtShapeProperties;

use crate::schemas::drawing::main::CtShapeStyle;

use crate::schemas::presentation::main::CtConnectorNonVisual;

use crate::schemas::presentation::main::CtExtensionListModify;

/**
 * @author : zhilong.zhou
 * @description : CT_Connector
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtConnector {
    #[serde(rename(serialize = "p:nvCxnSpPr", deserialize = "nvCxnSpPr"))]
    pub nv_cxn_sp_pr: CtConnectorNonVisual,

    #[serde(rename(serialize = "p:spPr", deserialize = "spPr"))]
    pub sp_pr: CtShapeProperties,

    #[serde(rename(serialize = "p:style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,
}
