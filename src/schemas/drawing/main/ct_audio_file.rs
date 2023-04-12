use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_AudioFile
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAudioFile {
    #[serde(rename = "@link")]
    pub link_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@contentType")]
    pub content_type_attr: Option<String>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
