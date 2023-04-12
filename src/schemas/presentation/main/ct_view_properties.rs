use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtNotesViewProperties;

use crate::schemas::presentation::main::CtNormalViewProperties;

use crate::schemas::presentation::main::CtOutlineViewProperties;

use crate::schemas::drawing::main::CtPositiveSize2D;

use crate::schemas::presentation::main::CtNotesTextViewProperties;

use crate::schemas::presentation::main::CtSlideSorterViewProperties;

use crate::schemas::presentation::main::CtSlideViewProperties;

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_ViewProperties
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:viewPr", deserialize = "viewPr"))]

pub struct CtViewProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lastView")]
    pub last_view_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showComments")]
    pub show_comments_attr: Option<String>,

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

    #[serde(rename(serialize = "p:normalViewPr", deserialize = "normalViewPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_view_pr: Option<CtNormalViewProperties>,

    #[serde(rename(serialize = "p:slideViewPr", deserialize = "slideViewPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slide_view_pr: Option<CtSlideViewProperties>,

    #[serde(rename(serialize = "p:outlineViewPr", deserialize = "outlineViewPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_view_pr: Option<CtOutlineViewProperties>,

    #[serde(rename(serialize = "p:notesTextViewPr", deserialize = "notesTextViewPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_text_view_pr: Option<CtNotesTextViewProperties>,

    #[serde(rename(serialize = "p:sorterViewPr", deserialize = "sorterViewPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorter_view_pr: Option<CtSlideSorterViewProperties>,

    #[serde(rename(serialize = "p:notesViewPr", deserialize = "notesViewPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_view_pr: Option<CtNotesViewProperties>,

    #[serde(rename(serialize = "p:gridSpacing", deserialize = "gridSpacing"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_spacing: Option<CtPositiveSize2D>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
