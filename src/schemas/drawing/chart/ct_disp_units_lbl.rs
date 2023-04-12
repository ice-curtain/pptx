use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtLayout;

use crate::schemas::drawing::chart::CtTx;

use crate::schemas::drawing::main::CtShapeProperties;

use crate::schemas::drawing::main::CtTextBody;

/**
 * @author : zhilong.zhou
 * @description : CT_DispUnitsLbl
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDispUnitsLbl {
    #[serde(rename(serialize = "layout", deserialize = "layout"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<CtLayout>,

    #[serde(rename(serialize = "tx", deserialize = "tx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<Box<CtTx>>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "txPr", deserialize = "txPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_pr: Option<Box<CtTextBody>>,
}
