use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtPictureFormat;

use crate::schemas::drawing::chart::CtPictureStackUnit;

use crate::schemas::drawing::chart::CtBoolean;

/**
 * @author : zhilong.zhou
 * @description : CT_PictureOptions
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPictureOptions {
    #[serde(rename(serialize = "applyToFront", deserialize = "applyToFront"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_to_front: Option<CtBoolean>,

    #[serde(rename(serialize = "applyToSides", deserialize = "applyToSides"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_to_sides: Option<CtBoolean>,

    #[serde(rename(serialize = "applyToEnd", deserialize = "applyToEnd"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_to_end: Option<CtBoolean>,

    #[serde(rename(serialize = "pictureFormat", deserialize = "pictureFormat"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_format: Option<CtPictureFormat>,

    #[serde(rename(serialize = "pictureStackUnit", deserialize = "pictureStackUnit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_stack_unit: Option<CtPictureStackUnit>,
}
