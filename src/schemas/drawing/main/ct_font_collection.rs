use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTextFont;

use crate::schemas::drawing::main::CtSupplementalFont;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_FontCollection
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtFontCollection {
    #[serde(rename(serialize = "a:latin", deserialize = "latin"))]
    pub latin: CtTextFont,

    #[serde(rename(serialize = "a:ea", deserialize = "ea"))]
    pub ea: CtTextFont,

    #[serde(rename(serialize = "a:cs", deserialize = "cs"))]
    pub cs: CtTextFont,

    #[serde(rename(serialize = "a:font", deserialize = "font"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Vec<CtSupplementalFont>>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
