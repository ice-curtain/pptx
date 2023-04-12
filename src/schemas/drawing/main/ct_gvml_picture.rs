use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtGvmlPictureNonVisual;

use crate::schemas::drawing::main::CtBlipFillProperties;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtShapeStyle;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_GvmlPicture
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGvmlPicture {
    #[serde(rename(serialize = "a:nvPicPr", deserialize = "nvPicPr"))]
    pub nv_pic_pr: CtGvmlPictureNonVisual,

    #[serde(rename(serialize = "a:blipFill", deserialize = "blipFill"))]
    pub blip_fill: Box<CtBlipFillProperties>,

    #[serde(rename(serialize = "a:spPr", deserialize = "spPr"))]
    pub sp_pr: Box<CtShapeProperties>,

    #[serde(rename(serialize = "a:style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
