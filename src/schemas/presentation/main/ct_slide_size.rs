use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_SlideSize
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSlideSize {
    #[serde(rename = "@cx")]
    pub cx_attr: String,

    #[serde(rename = "@cy")]
    pub cy_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type_attr: Option<String>,
}
