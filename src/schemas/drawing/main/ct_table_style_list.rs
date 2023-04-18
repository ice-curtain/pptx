use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTableStyle;

/**
 * @author : zhilong.zhou
 * @description : CT_TableStyleList
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "a:tblStyleLst", deserialize = "tblStyleLst"))]
pub struct CtTableStyleList {
    #[serde(rename = "@def")]
    pub def_attr: String,

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

    #[serde(rename(serialize = "a:tblStyle", deserialize = "tblStyle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbl_style: Option<Vec<CtTableStyle>>,
}
