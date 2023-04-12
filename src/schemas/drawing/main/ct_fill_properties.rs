use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtBlipFillProperties;

use crate::schemas::drawing::main::CtPatternFillProperties;

use crate::schemas::drawing::main::CtGradientFillProperties;

use crate::schemas::drawing::main::CtNoFillProperties;

use crate::schemas::drawing::main::CtSolidColorFillProperties;

use crate::schemas::drawing::main::CtGroupFillProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_FillProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtFillProperties {
    #[serde(rename(serialize = "a:noFill", deserialize = "noFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_fill: Option<CtNoFillProperties>,

    #[serde(rename(serialize = "a:solidFill", deserialize = "solidFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_fill: Option<CtSolidColorFillProperties>,

    #[serde(rename(serialize = "a:gradFill", deserialize = "gradFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grad_fill: Option<CtGradientFillProperties>,

    #[serde(rename(serialize = "a:blipFill", deserialize = "blipFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blip_fill: Option<Box<CtBlipFillProperties>>,

    #[serde(rename(serialize = "a:pattFill", deserialize = "pattFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patt_fill: Option<CtPatternFillProperties>,

    #[serde(rename(serialize = "a:grpFill", deserialize = "grpFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_fill: Option<CtGroupFillProperties>,
}
