use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAnimationGraphicalObjectBuildProperties;

use crate::schemas::presentation::main::CtEmpty;

/**
 * @author : zhilong.zhou
 * @description : CT_TLGraphicalObjectBuild
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlGraphicalObjectBuild {
    #[serde(rename = "@spid")]
    pub spid_attr: String,

    #[serde(rename = "@grpId")]
    pub grp_id_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@uiExpand")]
    pub ui_expand_attr: Option<String>,

    #[serde(rename(serialize = "p:bldAsOne", deserialize = "bldAsOne"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bld_as_one: Option<CtEmpty>,

    #[serde(rename(serialize = "p:bldSub", deserialize = "bldSub"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bld_sub: Option<CtAnimationGraphicalObjectBuildProperties>,
}
