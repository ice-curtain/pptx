use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_ColorMapping
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtColorMapping {
    #[serde(rename = "@bg1")]
    pub bg1_attr: String,

    #[serde(rename = "@tx1")]
    pub tx1_attr: String,

    #[serde(rename = "@bg2")]
    pub bg2_attr: String,

    #[serde(rename = "@tx2")]
    pub tx2_attr: String,

    #[serde(rename = "@accent1")]
    pub accent1_attr: String,

    #[serde(rename = "@accent2")]
    pub accent2_attr: String,

    #[serde(rename = "@accent3")]
    pub accent3_attr: String,

    #[serde(rename = "@accent4")]
    pub accent4_attr: String,

    #[serde(rename = "@accent5")]
    pub accent5_attr: String,

    #[serde(rename = "@accent6")]
    pub accent6_attr: String,

    #[serde(rename = "@hlink")]
    pub hlink_attr: String,

    #[serde(rename = "@folHlink")]
    pub fol_hlink_attr: String,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
