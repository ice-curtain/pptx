use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtDiagramDefinitionHeader;

/**
 * @author : zhilong.zhou
 * @description : CT_DiagramDefinitionHeaderLst
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "layoutDefHdrLst", deserialize = "layoutDefHdrLst"))]

pub struct CtDiagramDefinitionHeaderLst {
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

    #[serde(rename(serialize = "layoutDefHdr", deserialize = "layoutDefHdr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_def_hdr: Option<Vec<CtDiagramDefinitionHeader>>,
}
