use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::main::CtShapeProperties;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::main::CtTextBody;

/**
 * @author : zhilong.zhou
 * @description : CT_DTable
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDTable {
    #[serde(rename(serialize = "showHorzBorder", deserialize = "showHorzBorder"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_horz_border: Option<CtBoolean>,

    #[serde(rename(serialize = "showVertBorder", deserialize = "showVertBorder"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_vert_border: Option<CtBoolean>,

    #[serde(rename(serialize = "showOutline", deserialize = "showOutline"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_outline: Option<CtBoolean>,

    #[serde(rename(serialize = "showKeys", deserialize = "showKeys"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_keys: Option<CtBoolean>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "txPr", deserialize = "txPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_pr: Option<Box<CtTextBody>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
