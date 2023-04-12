use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Bevel
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBevel {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@w")]
    pub w_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@h")]
    pub h_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@prst")]
    pub prst_attr: Option<String>,
}
