use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtScene3D;

use crate::schemas::drawing::diagram::CtSdName;

use crate::schemas::drawing::diagram::CtStyleLabel;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::diagram::CtSdDescription;

use crate::schemas::drawing::diagram::CtSdCategories;

/**
 * @author : zhilong.zhou
 * @description : CT_StyleDefinition
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "styleDef", deserialize = "styleDef"))]

pub struct CtStyleDefinition {
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
    pub title: Option<Vec<CtSdName>>,

    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<Vec<CtSdDescription>>,

    #[serde(rename(serialize = "catLst", deserialize = "catLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_lst: Option<Vec<CtSdCategories>>,

    #[serde(rename(serialize = "scene3d", deserialize = "scene3d"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene3d: Option<CtScene3D>,

    #[serde(rename(serialize = "styleLbl", deserialize = "styleLbl"))]
    pub style_lbl: Vec<CtStyleLabel>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
