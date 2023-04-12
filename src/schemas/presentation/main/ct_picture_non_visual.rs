use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualPictureProperties;

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

use crate::schemas::presentation::main::CtApplicationNonVisualDrawingProps;

/**
 * @author : zhilong.zhou
 * @description : CT_PictureNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPictureNonVisual {
    #[serde(rename(serialize = "p:cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "p:cNvPicPr", deserialize = "cNvPicPr"))]
    pub c_nv_pic_pr: CtNonVisualPictureProperties,

    #[serde(rename(serialize = "p:nvPr", deserialize = "nvPr"))]
    pub nv_pr: CtApplicationNonVisualDrawingProps,
}
