use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAudioCdTime;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_AudioCD
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAudioCd {
    #[serde(rename(serialize = "a:st", deserialize = "st"))]
    pub st: CtAudioCdTime,

    #[serde(rename(serialize = "a:end", deserialize = "end"))]
    pub end: CtAudioCdTime,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
