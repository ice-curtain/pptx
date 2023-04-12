use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlTemplateList;

/**
 * @author : zhilong.zhou
 * @description : CT_TLBuildParagraph
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlBuildParagraph {
    #[serde(rename = "@spid")]
    pub spid_attr: String,

    #[serde(rename = "@grpId")]
    pub grp_id_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@uiExpand")]
    pub ui_expand_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@build")]
    pub build_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bldLvl")]
    pub bld_lvl_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@animBg")]
    pub anim_bg_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@autoUpdateAnimBg")]
    pub auto_update_anim_bg_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rev")]
    pub rev_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@advAuto")]
    pub adv_auto_attr: Option<String>,

    #[serde(rename(serialize = "p:tmplLst", deserialize = "tmplLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpl_lst: Option<CtTlTemplateList>,
}
