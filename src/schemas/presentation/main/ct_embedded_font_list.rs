use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtEmbeddedFontListEntry;

/**
 * @author : zhilong.zhou
 * @description : CT_EmbeddedFontList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtEmbeddedFontList {
    #[serde(rename(serialize = "p:embeddedFont", deserialize = "embeddedFont"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_font: Option<Vec<CtEmbeddedFontListEntry>>,
}
