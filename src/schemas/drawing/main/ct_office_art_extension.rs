use crate::schemas::standard::microsoft::drawing::CtThemeFamily;
use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_OfficeArtExtension
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtOfficeArtExtension {
    #[serde(rename = "@uri")]
    pub uri_attr: String,

    #[serde(rename(serialize = "thm15:themeFamily", deserialize = "themeFamily"))]
    theme_family: Option<CtThemeFamily>,
}
