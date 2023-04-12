use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtShapeStyle;

use crate::schemas::presentation::main::CtExtensionListModify;

use crate::schemas::drawing::main::CtShapeProperties;

use crate::schemas::presentation::main::CtShapeNonVisual;

use crate::schemas::drawing::main::CtTextBody;

/**
 * @author : zhilong.zhou
 * @description : CT_Shape
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtShape {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@useBgFill")]
    pub use_bg_fill_attr: Option<String>,

    #[serde(rename(serialize = "p:nvSpPr", deserialize = "nvSpPr"))]
    pub nv_sp_pr: CtShapeNonVisual,

    #[serde(rename(serialize = "p:spPr", deserialize = "spPr"))]
    pub sp_pr: CtShapeProperties,

    #[serde(rename(serialize = "p:style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,

    #[serde(rename(serialize = "p:txBody", deserialize = "txBody"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_body: Option<CtTextBody>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,
}
