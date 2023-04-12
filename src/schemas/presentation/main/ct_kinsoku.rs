use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Kinsoku
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtKinsoku {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lang")]
    pub lang_attr: Option<String>,

    #[serde(rename = "@invalStChars")]
    pub inval_st_chars_attr: String,

    #[serde(rename = "@invalEndChars")]
    pub inval_end_chars_attr: String,
}
