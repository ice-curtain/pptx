use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtPathShadeProperties;

use crate::schemas::drawing::main::CtRelativeRect;

use crate::schemas::drawing::main::CtGradientStopList;

use crate::schemas::drawing::main::CtLinearShadeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_GradientFillProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGradientFillProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@flip")]
    pub flip_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rotWithShape")]
    pub rot_with_shape_attr: Option<String>,

    #[serde(rename(serialize = "a:gsLst", deserialize = "gsLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gs_lst: Option<CtGradientStopList>,



    #[serde(rename(serialize = "a:lin", deserialize = "lin"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lin: Option<CtLinearShadeProperties>,

    #[serde(rename(serialize = "a:path", deserialize = "path"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<CtPathShadeProperties>,

    #[serde(rename(serialize = "a:tileRect", deserialize = "tileRect"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_rect: Option<CtRelativeRect>,
}
