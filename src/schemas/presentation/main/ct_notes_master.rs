use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionListModify;

use crate::schemas::presentation::main::CtCommonSlideData;

use crate::schemas::presentation::main::CtHeaderFooter;

use crate::schemas::drawing::main::CtColorMapping;

use crate::schemas::drawing::main::CtTextListStyle;

/**
 * @author : zhilong.zhou
 * @description : CT_NotesMaster
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:notesMaster", deserialize = "notesMaster"))]

pub struct CtNotesMaster {
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

    #[serde(rename(serialize = "p:hf", deserialize = "hf"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hf: Option<CtHeaderFooter>,

    #[serde(rename(serialize = "p:notesStyle", deserialize = "notesStyle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_style: Option<CtTextListStyle>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,

    #[serde(rename(serialize = "p:clrMap", deserialize = "clrMap"))]
    pub clr_map: CtColorMapping,
}
