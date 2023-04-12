use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtBaseStyles;

use crate::schemas::drawing::main::CtColorMapping;

/**
 * @author : zhilong.zhou
 * @description : CT_ClipboardStyleSheet
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtClipboardStyleSheet {
    #[serde(rename(serialize = "a:themeElements", deserialize = "themeElements"))]
    pub theme_elements: Box<CtBaseStyles>,

    #[serde(rename(serialize = "a:clrMap", deserialize = "clrMap"))]
    pub clr_map: CtColorMapping,
}
