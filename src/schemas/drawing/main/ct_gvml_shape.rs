use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtGvmlTextShape;

use crate::schemas::drawing::main::CtGvmlShapeNonVisual;

use crate::schemas::drawing::main::CtShapeStyle;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_GvmlShape
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGvmlShape {
    #[serde(rename(serialize = "a:nvSpPr", deserialize = "nvSpPr"))]
    pub nv_sp_pr: CtGvmlShapeNonVisual,

    #[serde(rename(serialize = "a:spPr", deserialize = "spPr"))]
    pub sp_pr: Box<CtShapeProperties>,

    #[serde(rename(serialize = "a:txSp", deserialize = "txSp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_sp: Option<Box<CtGvmlTextShape>>,

    #[serde(rename(serialize = "a:style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
