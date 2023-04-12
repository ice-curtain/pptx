use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLOleBuildChart
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlOleBuildChart {
    #[serde(rename = "@spid")]
    pub spid_attr: String,

    #[serde(rename = "@grpId")]
    pub grp_id_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@uiExpand")]
    pub ui_expand_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bld")]
    pub bld_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@animBg")]
    pub anim_bg_attr: Option<String>,
}
