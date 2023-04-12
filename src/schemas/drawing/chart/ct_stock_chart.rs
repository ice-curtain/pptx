use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtLineSer;

use crate::schemas::drawing::chart::CtDLbls;

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtUpDownBars;

use crate::schemas::drawing::chart::CtChartLines;

/**
 * @author : zhilong.zhou
 * @description : CT_StockChart
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtStockChart {
    #[serde(rename(serialize = "ser", deserialize = "ser"))]
    pub ser: Vec<CtLineSer>,

    #[serde(rename(serialize = "dLbls", deserialize = "dLbls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_lbls: Option<Box<CtDLbls>>,

    #[serde(rename(serialize = "dropLines", deserialize = "dropLines"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_lines: Option<Box<CtChartLines>>,

    #[serde(rename(serialize = "hiLowLines", deserialize = "hiLowLines"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hi_low_lines: Option<Box<CtChartLines>>,

    #[serde(rename(serialize = "upDownBars", deserialize = "upDownBars"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_down_bars: Option<Box<CtUpDownBars>>,

    #[serde(rename(serialize = "axId", deserialize = "axId"))]
    pub ax_id: Vec<CtUnsignedInt>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
