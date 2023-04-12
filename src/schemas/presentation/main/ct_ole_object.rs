use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtOleObjectEmbed;

use crate::schemas::presentation::main::CtPicture;

use crate::schemas::presentation::main::CtOleObjectLink;

/**
 * @author : zhilong.zhou
 * @description : CT_OleObject
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:oleObj", deserialize = "oleObj"))]

pub struct CtOleObject {
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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@progId")]
    pub prog_id_attr: Option<String>,

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

    #[serde(rename(serialize = "p:pic", deserialize = "pic"))]
    pub pic: CtPicture,

    #[serde(rename(serialize = "p:embed", deserialize = "embed"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<CtOleObjectEmbed>,

    #[serde(rename(serialize = "p:link", deserialize = "link"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CtOleObjectLink>,
}
