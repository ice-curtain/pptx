use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtDLbls;

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtGrouping;

use crate::schemas::drawing::chart::CtAreaSer;

use crate::schemas::drawing::chart::CtChartLines;

/**
 * @author : zhilong.zhou
 * @description : CT_AreaChart
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAreaChart {
    #[serde(rename(serialize = "axId", deserialize = "axId"))]
    pub ax_id: Vec<CtUnsignedInt>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "grouping", deserialize = "grouping"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<CtGrouping>,

    #[serde(rename(serialize = "varyColors", deserialize = "varyColors"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vary_colors: Option<CtBoolean>,

    #[serde(rename(serialize = "ser", deserialize = "ser"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ser: Option<Vec<CtAreaSer>>,

    #[serde(rename(serialize = "dLbls", deserialize = "dLbls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_lbls: Option<Box<CtDLbls>>,

    #[serde(rename(serialize = "dropLines", deserialize = "dropLines"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_lines: Option<Box<CtChartLines>>,
}
