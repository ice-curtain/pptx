use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_SupplementalFont
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSupplementalFont {
    #[serde(rename = "@script")]
    pub script_attr: String,

    #[serde(rename = "@typeface")]
    pub typeface_attr: String,
}
