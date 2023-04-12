use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTableStyleTextStyle;

use crate::schemas::drawing::main::CtTableStyleCellStyle;

/**
 * @author : zhilong.zhou
 * @description : CT_TablePartStyle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTablePartStyle {
    #[serde(rename(serialize = "a:tcTxStyle", deserialize = "tcTxStyle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc_tx_style: Option<CtTableStyleTextStyle>,

    #[serde(rename(serialize = "a:tcStyle", deserialize = "tcStyle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc_style: Option<Box<CtTableStyleCellStyle>>,
}
