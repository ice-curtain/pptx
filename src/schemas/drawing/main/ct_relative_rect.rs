use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_RelativeRect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtRelativeRect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@l")]
    pub l_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@t")]
    pub t_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@r")]
    pub r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@b")]
    pub b_attr: Option<String>,
}
