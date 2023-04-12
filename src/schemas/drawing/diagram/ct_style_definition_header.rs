use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtSdName;

use crate::schemas::drawing::diagram::CtSdDescription;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::diagram::CtSdCategories;

/**
 * @author : zhilong.zhou
 * @description : CT_StyleDefinitionHeader
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "styleDefHdr", deserialize = "styleDefHdr"))]

pub struct CtStyleDefinitionHeader {
    #[serde(rename = "@uniqueId")]
    pub unique_id_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@minVer")]
    pub min_ver_attr: Option<String>,

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
    pub title: Vec<CtSdName>,

    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    pub desc: Vec<CtSdDescription>,

    #[serde(rename(serialize = "catLst", deserialize = "catLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_lst: Option<Vec<CtSdCategories>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
