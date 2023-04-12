use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLByRgbColorTransform
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlByRgbColorTransform {
    #[serde(rename = "@r")]
    pub r_attr: String,

    #[serde(rename = "@g")]
    pub g_attr: String,

    #[serde(rename = "@b")]
    pub b_attr: String,
}
