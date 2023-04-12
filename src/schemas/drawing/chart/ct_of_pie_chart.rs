use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtOfPieType;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtSecondPieSize;

use crate::schemas::drawing::chart::CtDLbls;

use crate::schemas::drawing::chart::CtSplitType;

use crate::schemas::drawing::chart::CtDouble;

use crate::schemas::drawing::chart::CtChartLines;

use crate::schemas::drawing::chart::CtPieSer;

use crate::schemas::drawing::chart::CtCustSplit;

use crate::schemas::drawing::chart::CtGapAmount;

/**
 * @author : zhilong.zhou
 * @description : CT_OfPieChart
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtOfPieChart {
    #[serde(rename(serialize = "ofPieType", deserialize = "ofPieType"))]
    pub of_pie_type: CtOfPieType,

    #[serde(rename(serialize = "gapWidth", deserialize = "gapWidth"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_width: Option<CtGapAmount>,

    #[serde(rename(serialize = "splitType", deserialize = "splitType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_type: Option<CtSplitType>,

    #[serde(rename(serialize = "splitPos", deserialize = "splitPos"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_pos: Option<CtDouble>,

    #[serde(rename(serialize = "custSplit", deserialize = "custSplit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_split: Option<CtCustSplit>,

    #[serde(rename(serialize = "secondPieSize", deserialize = "secondPieSize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_pie_size: Option<CtSecondPieSize>,

    #[serde(rename(serialize = "serLines", deserialize = "serLines"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ser_lines: Option<Vec<CtChartLines>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "varyColors", deserialize = "varyColors"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vary_colors: Option<CtBoolean>,

    #[serde(rename(serialize = "ser", deserialize = "ser"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ser: Option<Vec<CtPieSer>>,

    #[serde(rename(serialize = "dLbls", deserialize = "dLbls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_lbls: Option<Box<CtDLbls>>,
}
