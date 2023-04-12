use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtColorScheme;

use crate::schemas::drawing::main::CtColorMapping;

/**
 * @author : zhilong.zhou
 * @description : CT_ColorSchemeAndMapping
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtColorSchemeAndMapping {
    #[serde(rename(serialize = "a:clrScheme", deserialize = "clrScheme"))]
    pub clr_scheme: CtColorScheme,

    #[serde(rename(serialize = "a:clrMap", deserialize = "clrMap"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_map: Option<CtColorMapping>,
}
