use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_PhotoAlbum
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPhotoAlbum {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bw")]
    pub bw_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showCaptions")]
    pub show_captions_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@layout")]
    pub layout_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@frame")]
    pub frame_attr: Option<String>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
