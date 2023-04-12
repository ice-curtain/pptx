use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtColor;

/**
 * @author : zhilong.zhou
 * @description : CT_PatternFillProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPatternFillProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@prst")]
    pub prst_attr: Option<String>,

    #[serde(rename(serialize = "a:fgClr", deserialize = "fgClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fg_clr: Option<CtColor>,

    #[serde(rename(serialize = "a:bgClr", deserialize = "bgClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg_clr: Option<CtColor>,
}
