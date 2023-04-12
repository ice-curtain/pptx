use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_PageMargins
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPageMargins {
    #[serde(rename = "@l")]
    pub l_attr: String,

    #[serde(rename = "@r")]
    pub r_attr: String,

    #[serde(rename = "@t")]
    pub t_attr: String,

    #[serde(rename = "@b")]
    pub b_attr: String,

    #[serde(rename = "@header")]
    pub header_attr: String,

    #[serde(rename = "@footer")]
    pub footer_attr: String,
}
