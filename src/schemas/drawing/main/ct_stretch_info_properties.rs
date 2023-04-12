use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtRelativeRect;

/**
 * @author : zhilong.zhou
 * @description : CT_StretchInfoProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtStretchInfoProperties {
    #[serde(rename(serialize = "a:fillRect", deserialize = "fillRect"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_rect: Option<CtRelativeRect>,
}
