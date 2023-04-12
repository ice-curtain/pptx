use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtTextBodyProperties;

use crate::schemas::drawing::main::CtTextListStyle;

use crate::schemas::drawing::main::CtShapeStyle;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_DefaultShapeDefinition
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDefaultShapeDefinition {
    #[serde(rename(serialize = "a:spPr", deserialize = "spPr"))]
    pub sp_pr: Box<CtShapeProperties>,

    #[serde(rename(serialize = "a:bodyPr", deserialize = "bodyPr"))]
    pub body_pr: CtTextBodyProperties,

    #[serde(rename(serialize = "a:lstStyle", deserialize = "lstStyle"))]
    pub lst_style: Box<CtTextListStyle>,

    #[serde(rename(serialize = "a:style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
