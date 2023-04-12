use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualPictureProperties;

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

/**
 * @author : zhilong.zhou
 * @description : CT_PictureNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPictureNonVisual {
    #[serde(rename(serialize = "cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "cNvPicPr", deserialize = "cNvPicPr"))]
    pub c_nv_pic_pr: CtNonVisualPictureProperties,
}
