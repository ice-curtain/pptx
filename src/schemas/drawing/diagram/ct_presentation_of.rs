use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_PresentationOf
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPresentationOf {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@axis")]
    pub axis_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@ptType")]
    pub pt_type_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hideLastTrans")]
    pub hide_last_trans_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@st")]
    pub st_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@cnt")]
    pub cnt_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@step")]
    pub step_attr: Option<String>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
