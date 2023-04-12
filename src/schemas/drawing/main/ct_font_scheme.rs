use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtFontCollection;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_FontScheme
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtFontScheme {
    #[serde(rename = "@name")]
    pub name_attr: String,

    #[serde(rename(serialize = "a:majorFont", deserialize = "majorFont"))]
    pub major_font: CtFontCollection,

    #[serde(rename(serialize = "a:minorFont", deserialize = "minorFont"))]
    pub minor_font: CtFontCollection,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
