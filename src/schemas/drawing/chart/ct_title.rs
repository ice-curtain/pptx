use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::main::CtTextBody;

use crate::schemas::drawing::chart::CtTx;

use crate::schemas::drawing::chart::CtLayout;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_Title
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTitle {
    #[serde(rename(serialize = "tx", deserialize = "tx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<Box<CtTx>>,

    #[serde(rename(serialize = "layout", deserialize = "layout"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<CtLayout>,

    #[serde(rename(serialize = "overlay", deserialize = "overlay"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<CtBoolean>,

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
