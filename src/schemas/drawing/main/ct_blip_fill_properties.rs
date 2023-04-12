use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtBlip;

use crate::schemas::drawing::main::CtRelativeRect;

use crate::schemas::drawing::main::CtStretchInfoProperties;

use crate::schemas::drawing::main::CtTileInfoProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_BlipFillProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBlipFillProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dpi")]
    pub dpi_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rotWithShape")]
    pub rot_with_shape_attr: Option<String>,

    #[serde(rename(serialize = "a:blip", deserialize = "blip"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blip: Option<Box<CtBlip>>,

    #[serde(rename(serialize = "a:srcRect", deserialize = "srcRect"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_rect: Option<CtRelativeRect>,

    #[serde(rename(serialize = "a:tile", deserialize = "tile"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile: Option<CtTileInfoProperties>,

    #[serde(rename(serialize = "a:stretch", deserialize = "stretch"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stretch: Option<CtStretchInfoProperties>,
}
