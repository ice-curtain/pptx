use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLByHslColorTransform
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlByHslColorTransform {
    #[serde(rename = "@h")]
    pub h_attr: String,

    #[serde(rename = "@s")]
    pub s_attr: String,

    #[serde(rename = "@l")]
    pub l_attr: String,
}
