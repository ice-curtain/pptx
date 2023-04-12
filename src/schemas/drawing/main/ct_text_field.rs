use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTextParagraphProperties;

use crate::schemas::drawing::main::CtTextCharacterProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_TextField
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextField {
    #[serde(rename = "@id")]
    pub id_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type_attr: Option<String>,

    #[serde(rename(serialize = "a:rPr", deserialize = "rPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_pr: Option<Box<CtTextCharacterProperties>>,

    #[serde(rename(serialize = "a:pPr", deserialize = "pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:t", deserialize = "t"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
}
