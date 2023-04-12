use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtElemPropSet;

use crate::schemas::drawing::main::CtShapeProperties;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtTextBody;

/**
 * @author : zhilong.zhou
 * @description : CT_Pt
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPt {
    #[serde(rename = "@modelId")]
    pub model_id_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@cxnId")]
    pub cxn_id_attr: Option<String>,

    #[serde(rename(serialize = "prSet", deserialize = "prSet"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr_set: Option<CtElemPropSet>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "t", deserialize = "t"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<Box<CtTextBody>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
