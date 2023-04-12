use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chartDrawing::CtConnectorNonVisual;

use crate::schemas::drawing::main::CtShapeStyle;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_Connector
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtConnector {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@macro")]
    pub r#macro_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fPublished")]
    pub f_published_attr: Option<String>,

    #[serde(rename(serialize = "nvCxnSpPr", deserialize = "nvCxnSpPr"))]
    pub nv_cxn_sp_pr: CtConnectorNonVisual,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    pub sp_pr: Box<CtShapeProperties>,

    #[serde(rename(serialize = "style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,
}
