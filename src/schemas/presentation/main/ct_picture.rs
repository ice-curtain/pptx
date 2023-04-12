use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtShapeStyle;

use crate::schemas::presentation::main::CtExtensionListModify;

use crate::schemas::drawing::main::CtBlipFillProperties;

use crate::schemas::presentation::main::CtPictureNonVisual;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_Picture
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:pic", deserialize = "pic"))]

pub struct CtPicture {
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

    #[serde(rename(serialize = "p:nvPicPr", deserialize = "nvPicPr"))]
    pub nv_pic_pr: CtPictureNonVisual,

    #[serde(rename(serialize = "p:blipFill", deserialize = "blipFill"))]
    pub blip_fill: CtBlipFillProperties,

    #[serde(rename(serialize = "p:spPr", deserialize = "spPr"))]
    pub sp_pr: CtShapeProperties,

    #[serde(rename(serialize = "p:style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,
}
