use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chartDrawing::CtPictureNonVisual;

use crate::schemas::drawing::main::CtShapeStyle;

use crate::schemas::drawing::main::CtBlipFillProperties;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_Picture
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "pic", deserialize = "pic"))]

pub struct CtPicture {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@macro")]
    pub r#macro_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fPublished")]
    pub f_published_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:p")]
    pub p_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:a")]
    pub a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:r")]
    pub r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:s")]
    pub s_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns")]
    pub default_namespace_attr: Option<String>,

    #[serde(rename(serialize = "nvPicPr", deserialize = "nvPicPr"))]
    pub nv_pic_pr: CtPictureNonVisual,

    #[serde(rename(serialize = "blipFill", deserialize = "blipFill"))]
    pub blip_fill: Box<CtBlipFillProperties>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    pub sp_pr: Box<CtShapeProperties>,

    #[serde(rename(serialize = "style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,
}
