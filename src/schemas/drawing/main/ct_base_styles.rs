use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtFontScheme;

use crate::schemas::drawing::main::CtColorScheme;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtStyleMatrix;

/**
 * @author : zhilong.zhou
 * @description : CT_BaseStyles
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBaseStyles {
    #[serde(rename(serialize = "a:clrScheme", deserialize = "clrScheme"))]
    pub clr_scheme: CtColorScheme,

    #[serde(rename(serialize = "a:fontScheme", deserialize = "fontScheme"))]
    pub font_scheme: CtFontScheme,

    #[serde(rename(serialize = "a:fmtScheme", deserialize = "fmtScheme"))]
    pub fmt_scheme: Box<CtStyleMatrix>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
