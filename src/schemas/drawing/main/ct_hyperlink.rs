use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtEmbeddedWavAudioFile;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_Hyperlink
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtHyperlink {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@id")]
    pub id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@invalidUrl")]
    pub invalid_url_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@action")]
    pub action_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@tgtFrame")]
    pub tgt_frame_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@tooltip")]
    pub tooltip_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@history")]
    pub history_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@highlightClick")]
    pub highlight_click_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@endSnd")]
    pub end_snd_attr: Option<String>,

    #[serde(rename(serialize = "a:snd", deserialize = "snd"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snd: Option<CtEmbeddedWavAudioFile>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
