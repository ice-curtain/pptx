use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtCtCategories;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::diagram::CtCtName;

use crate::schemas::drawing::diagram::CtCtStyleLabel;

use crate::schemas::drawing::diagram::CtCtDescription;

/**
 * @author : zhilong.zhou
 * @description : CT_ColorTransform
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "colorsDef", deserialize = "colorsDef"))]

pub struct CtColorTransform {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@uniqueId")]
    pub unique_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@minVer")]
    pub min_ver_attr: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<CtCtName>>,

    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<Vec<CtCtDescription>>,

    #[serde(rename(serialize = "catLst", deserialize = "catLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_lst: Option<Vec<CtCtCategories>>,

    #[serde(rename(serialize = "styleLbl", deserialize = "styleLbl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_lbl: Option<Vec<CtCtStyleLabel>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
