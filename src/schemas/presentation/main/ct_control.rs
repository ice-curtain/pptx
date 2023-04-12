use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

use crate::schemas::presentation::main::CtPicture;

/**
 * @author : zhilong.zhou
 * @description : CT_Control
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtControl {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@name")]
    pub name_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showAsIcon")]
    pub show_as_icon_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@r:id")]
    pub r_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@imgW")]
    pub img_w_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@imgH")]
    pub img_h_attr: Option<String>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "p:pic", deserialize = "pic"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic: Option<CtPicture>,
}
