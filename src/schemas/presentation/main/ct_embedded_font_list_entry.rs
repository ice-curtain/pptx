use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtEmbeddedFontDataId;

use crate::schemas::drawing::main::CtTextFont;

/**
 * @author : zhilong.zhou
 * @description : CT_EmbeddedFontListEntry
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtEmbeddedFontListEntry {
    #[serde(rename(serialize = "p:font", deserialize = "font"))]
    pub font: CtTextFont,

    #[serde(rename(serialize = "p:regular", deserialize = "regular"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular: Option<CtEmbeddedFontDataId>,

    #[serde(rename(serialize = "p:bold", deserialize = "bold"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<CtEmbeddedFontDataId>,

    #[serde(rename(serialize = "p:italic", deserialize = "italic"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<CtEmbeddedFontDataId>,

    #[serde(rename(serialize = "p:boldItalic", deserialize = "boldItalic"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold_italic: Option<CtEmbeddedFontDataId>,
}
