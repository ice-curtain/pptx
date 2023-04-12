use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtRelativeRect;

/**
 * @author : zhilong.zhou
 * @description : CT_PathShadeProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPathShadeProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@path")]
    pub path_attr: Option<String>,

    #[serde(rename(serialize = "a:fillToRect", deserialize = "fillToRect"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_to_rect: Option<CtRelativeRect>,
}
