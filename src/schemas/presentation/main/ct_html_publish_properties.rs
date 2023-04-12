use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtIndexRange;

use crate::schemas::presentation::main::CtCustomShowId;

use crate::schemas::presentation::main::CtExtensionList;

use crate::schemas::presentation::main::CtEmpty;

/**
 * @author : zhilong.zhou
 * @description : CT_HtmlPublishProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtHtmlPublishProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showSpeakerNotes")]
    pub show_speaker_notes_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@target")]
    pub target_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@title")]
    pub title_attr: Option<String>,

    #[serde(rename = "@r:id")]
    pub r_id_attr: String,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "p:sldAll", deserialize = "sldAll"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_all: Option<CtEmpty>,

    #[serde(rename(serialize = "p:sldRg", deserialize = "sldRg"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_rg: Option<CtIndexRange>,

    #[serde(rename(serialize = "p:custShow", deserialize = "custShow"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_show: Option<CtCustomShowId>,
}
