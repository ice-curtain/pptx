use crate::schemas::drawing::main::CtOfficeArtExtensionList;
use serde::{Deserialize, Serialize};
/**
 * xmlns:thm15=http://schemas.microsoft.com/office/thememl/2012/main
 */

/**
 * @author : link@lsx
 * @date : 2023/4/7 11:25
 * @description :
 */
/**
 * @author : zhilong.zhou
 * @description : CT_ThemeFamily
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CtThemeFamily {
    #[serde(rename = "@xmlns:thm15")]
    thm15: Option<String>,

    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@vid")]
    vid: String,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    ext_lst: Box<Option<CtOfficeArtExtensionList>>,
}
