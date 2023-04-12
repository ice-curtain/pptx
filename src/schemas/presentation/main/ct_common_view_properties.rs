use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtScale2D;

use crate::schemas::drawing::main::CtPoint2D;

/**
 * @author : zhilong.zhou
 * @description : CT_CommonViewProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCommonViewProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@varScale")]
    pub var_scale_attr: Option<String>,

    #[serde(rename(serialize = "p:scale", deserialize = "scale"))]
    pub scale: CtScale2D,

    #[serde(rename(serialize = "p:origin", deserialize = "origin"))]
    pub origin: CtPoint2D,
}
