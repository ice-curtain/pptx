use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::diagram::CtName;

use crate::schemas::drawing::diagram::CtCategories;

use crate::schemas::drawing::diagram::CtDescription;

/**
 * @author : zhilong.zhou
 * @description : CT_DiagramDefinitionHeader
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "layoutDefHdr", deserialize = "layoutDefHdr"))]

pub struct CtDiagramDefinitionHeader {
    #[serde(rename = "@uniqueId")]
    pub unique_id_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@minVer")]
    pub min_ver_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@defStyle")]
    pub def_style_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@resId")]
    pub res_id_attr: Option<String>,

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

    #[serde(rename(serialize = "title", deserialize = "title"))]
    pub title: Vec<CtName>,

    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    pub desc: Vec<CtDescription>,

    #[serde(rename(serialize = "catLst", deserialize = "catLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_lst: Option<Vec<CtCategories>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
