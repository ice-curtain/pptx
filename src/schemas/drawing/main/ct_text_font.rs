use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TextFont
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextFont {
    #[serde(rename = "@typeface")]
    pub typeface_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@panose")]
    pub panose_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@pitchFamily")]
    pub pitch_family_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@charset")]
    pub charset_attr: Option<String>,
}
