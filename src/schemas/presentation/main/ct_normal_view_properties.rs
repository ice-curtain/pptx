use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

use crate::schemas::presentation::main::CtNormalViewPortion;

/**
 * @author : zhilong.zhou
 * @description : CT_NormalViewProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNormalViewProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showOutlineIcons")]
    pub show_outline_icons_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@snapVertSplitter")]
    pub snap_vert_splitter_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@vertBarState")]
    pub vert_bar_state_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@horzBarState")]
    pub horz_bar_state_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@preferSingleView")]
    pub prefer_single_view_attr: Option<String>,

    #[serde(rename(serialize = "p:restoredLeft", deserialize = "restoredLeft"))]
    pub restored_left: CtNormalViewPortion,

    #[serde(rename(serialize = "p:restoredTop", deserialize = "restoredTop"))]
    pub restored_top: CtNormalViewPortion,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
