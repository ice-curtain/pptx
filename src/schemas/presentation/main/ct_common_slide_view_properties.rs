use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtGuideList;

use crate::schemas::presentation::main::CtCommonViewProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_CommonSlideViewProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCommonSlideViewProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@snapToGrid")]
    pub snap_to_grid_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@snapToObjects")]
    pub snap_to_objects_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showGuides")]
    pub show_guides_attr: Option<String>,

    #[serde(rename(serialize = "p:cViewPr", deserialize = "cViewPr"))]
    pub c_view_pr: CtCommonViewProperties,

    #[serde(rename(serialize = "p:guideLst", deserialize = "guideLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_lst: Option<CtGuideList>,
}
