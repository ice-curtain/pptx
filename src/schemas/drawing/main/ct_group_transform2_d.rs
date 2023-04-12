use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtPoint2D;

use crate::schemas::drawing::main::CtPositiveSize2D;

/**
 * @author : zhilong.zhou
 * @description : CT_GroupTransform2D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGroupTransform2D {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rot")]
    pub rot_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@flipH")]
    pub flip_h_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@flipV")]
    pub flip_v_attr: Option<String>,

    #[serde(rename(serialize = "a:off", deserialize = "off"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off: Option<CtPoint2D>,

    #[serde(rename(serialize = "a:ext", deserialize = "ext"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<CtPositiveSize2D>,

    #[serde(rename(serialize = "a:chOff", deserialize = "chOff"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_off: Option<CtPoint2D>,

    #[serde(rename(serialize = "a:chExt", deserialize = "chExt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_ext: Option<CtPositiveSize2D>,
}
