use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtNonVisualPictureProperties;

use crate::schemas::drawing::main::CtNonVisualDrawingProps;

/**
 * @author : zhilong.zhou
 * @description : CT_GvmlPictureNonVisual
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGvmlPictureNonVisual {
    #[serde(rename(serialize = "a:cNvPr", deserialize = "cNvPr"))]
    pub c_nv_pr: CtNonVisualDrawingProps,

    #[serde(rename(serialize = "a:cNvPicPr", deserialize = "cNvPicPr"))]
    pub c_nv_pic_pr: CtNonVisualPictureProperties,
}
