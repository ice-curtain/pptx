use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_LineEndProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLineEndProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@w")]
    pub w_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@len")]
    pub len_attr: Option<String>,
}
