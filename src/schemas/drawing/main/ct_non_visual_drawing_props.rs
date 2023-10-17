use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtHyperlink;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_NonVisualDrawingProps
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNonVisualDrawingProps {
    #[serde(rename = "@id")]
    pub id_attr: u32,

    #[serde(rename = "@name")]
    pub name_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@descr")]
    pub descr_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hidden")]
    pub hidden_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@title")]
    pub title_attr: Option<String>,

    #[serde(rename(serialize = "a:hlinkClick", deserialize = "hlinkClick"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hlink_click: Option<CtHyperlink>,

    #[serde(rename(serialize = "a:hlinkHover", deserialize = "hlinkHover"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hlink_hover: Option<CtHyperlink>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
