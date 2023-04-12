use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_ContentPartLocking
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtContentPartLocking {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@noGrp")]
    pub no_grp_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@noSelect")]
    pub no_select_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@noRot")]
    pub no_rot_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@noChangeAspect")]
    pub no_change_aspect_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@noMove")]
    pub no_move_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@noResize")]
    pub no_resize_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@noEditPoints")]
    pub no_edit_points_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@noAdjustHandles")]
    pub no_adjust_handles_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@noChangeArrowheads")]
    pub no_change_arrowheads_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@noChangeShapeType")]
    pub no_change_shape_type_attr: Option<String>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
