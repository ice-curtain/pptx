use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtUpDownBar;

use crate::schemas::drawing::chart::CtGapAmount;

/**
 * @author : zhilong.zhou
 * @description : CT_UpDownBars
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtUpDownBars {
    #[serde(rename(serialize = "gapWidth", deserialize = "gapWidth"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_width: Option<CtGapAmount>,

    #[serde(rename(serialize = "upBars", deserialize = "upBars"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_bars: Option<Box<CtUpDownBar>>,

    #[serde(rename(serialize = "downBars", deserialize = "downBars"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_bars: Option<Box<CtUpDownBar>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
