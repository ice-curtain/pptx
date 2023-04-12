use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtColor;

/**
 * @author : zhilong.zhou
 * @description : CT_ColorChangeEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtColorChangeEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@useA")]
    pub use_a_attr: Option<String>,

    #[serde(rename(serialize = "a:clrFrom", deserialize = "clrFrom"))]
    pub clr_from: CtColor,

    #[serde(rename(serialize = "a:clrTo", deserialize = "clrTo"))]
    pub clr_to: CtColor,
}
