use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtMarker;

use crate::schemas::drawing::chart::CtPictureOptions;

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_DPt
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDPt {
    #[serde(rename(serialize = "idx", deserialize = "idx"))]
    pub idx: CtUnsignedInt,

    #[serde(rename(serialize = "invertIfNegative", deserialize = "invertIfNegative"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert_if_negative: Option<CtBoolean>,

    #[serde(rename(serialize = "marker", deserialize = "marker"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<Box<CtMarker>>,

    #[serde(rename(serialize = "bubble3D", deserialize = "bubble3D"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bubble3_d: Option<CtBoolean>,

    #[serde(rename(serialize = "explosion", deserialize = "explosion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explosion: Option<CtUnsignedInt>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "pictureOptions", deserialize = "pictureOptions"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_options: Option<CtPictureOptions>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
