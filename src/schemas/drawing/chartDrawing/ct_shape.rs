use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chartDrawing::CtShapeNonVisual;

use crate::schemas::drawing::main::CtShapeStyle;

use crate::schemas::drawing::main::CtShapeProperties;

use crate::schemas::drawing::main::CtTextBody;

/**
 * @author : zhilong.zhou
 * @description : CT_Shape
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtShape {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@macro")]
    pub r#macro_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@textlink")]
    pub textlink_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fLocksText")]
    pub f_locks_text_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fPublished")]
    pub f_published_attr: Option<String>,

    #[serde(rename(serialize = "nvSpPr", deserialize = "nvSpPr"))]
    pub nv_sp_pr: CtShapeNonVisual,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    pub sp_pr: Box<CtShapeProperties>,

    #[serde(rename(serialize = "style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,

    #[serde(rename(serialize = "txBody", deserialize = "txBody"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_body: Option<Box<CtTextBody>>,
}
