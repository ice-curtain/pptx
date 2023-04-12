use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtStyleDefinitionHeader;

/**
 * @author : zhilong.zhou
 * @description : CT_StyleDefinitionHeaderLst
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "styleDefHdrLst", deserialize = "styleDefHdrLst"))]

pub struct CtStyleDefinitionHeaderLst {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:p")]
    pub p_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:a")]
    pub a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:r")]
    pub r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:s")]
    pub s_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns")]
    pub default_namespace_attr: Option<String>,

    #[serde(rename(serialize = "styleDefHdr", deserialize = "styleDefHdr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_def_hdr: Option<Vec<CtStyleDefinitionHeader>>,
}
