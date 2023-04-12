use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::diagram::CtName;

use crate::schemas::drawing::diagram::CtSampleData;

use crate::schemas::drawing::diagram::CtLayoutNode;

use crate::schemas::drawing::diagram::CtCategories;

use crate::schemas::drawing::diagram::CtDescription;

/**
 * @author : zhilong.zhou
 * @description : CT_DiagramDefinition
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "layoutDef", deserialize = "layoutDef"))]

pub struct CtDiagramDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@uniqueId")]
    pub unique_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@minVer")]
    pub min_ver_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@defStyle")]
    pub def_style_attr: Option<String>,

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
    pub title: Option<Vec<CtName>>,

    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<Vec<CtDescription>>,

    #[serde(rename(serialize = "catLst", deserialize = "catLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_lst: Option<Vec<CtCategories>>,

    #[serde(rename(serialize = "sampData", deserialize = "sampData"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samp_data: Option<Vec<CtSampleData>>,

    #[serde(rename(serialize = "styleData", deserialize = "styleData"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_data: Option<Vec<CtSampleData>>,

    #[serde(rename(serialize = "clrData", deserialize = "clrData"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_data: Option<Vec<CtSampleData>>,

    #[serde(rename(serialize = "layoutNode", deserialize = "layoutNode"))]
    pub layout_node: Vec<CtLayoutNode>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
