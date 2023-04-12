use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtFillProperties;

use crate::schemas::drawing::main::CtCell3D;

use crate::schemas::drawing::main::CtTableCellBorderStyle;

use crate::schemas::drawing::main::CtStyleMatrixReference;

/**
 * @author : zhilong.zhou
 * @description : CT_TableStyleCellStyle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTableStyleCellStyle {
    #[serde(rename(serialize = "a:tcBdr", deserialize = "tcBdr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc_bdr: Option<CtTableCellBorderStyle>,

    #[serde(rename(serialize = "a:cell3D", deserialize = "cell3D"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell3_d: Option<CtCell3D>,

    #[serde(rename(serialize = "a:fill", deserialize = "fill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Box<CtFillProperties>>,

    #[serde(rename(serialize = "a:fillRef", deserialize = "fillRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_ref: Option<CtStyleMatrixReference>,
}
