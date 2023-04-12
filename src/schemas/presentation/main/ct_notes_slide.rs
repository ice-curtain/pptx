use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionListModify;

use crate::schemas::presentation::main::CtCommonSlideData;

use crate::schemas::drawing::main::CtColorMappingOverride;

/**
 * @author : zhilong.zhou
 * @description : CT_NotesSlide
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:notes", deserialize = "notes"))]

pub struct CtNotesSlide {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showMasterSp")]
    pub show_master_sp_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showMasterPhAnim")]
    pub show_master_ph_anim_attr: Option<String>,

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

    #[serde(rename(serialize = "p:cSld", deserialize = "cSld"))]
    pub c_sld: CtCommonSlideData,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,

    #[serde(rename(serialize = "p:clrMapOvr", deserialize = "clrMapOvr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_map_ovr: Option<CtColorMappingOverride>,
}
