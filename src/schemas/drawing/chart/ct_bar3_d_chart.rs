use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtShape;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtBarDir;

use crate::schemas::drawing::chart::CtBarGrouping;

use crate::schemas::drawing::chart::CtDLbls;

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtBarSer;

use crate::schemas::drawing::chart::CtGapAmount;

/**
 * @author : zhilong.zhou
 * @description : CT_Bar3DChart
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBar3DChart {
    #[serde(rename(serialize = "gapWidth", deserialize = "gapWidth"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_width: Option<CtGapAmount>,

    #[serde(rename(serialize = "gapDepth", deserialize = "gapDepth"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_depth: Option<CtGapAmount>,

    #[serde(rename(serialize = "shape", deserialize = "shape"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<CtShape>,

    #[serde(rename(serialize = "axId", deserialize = "axId"))]
    pub ax_id: Vec<CtUnsignedInt>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "barDir", deserialize = "barDir"))]
    pub bar_dir: CtBarDir,

    #[serde(rename(serialize = "grouping", deserialize = "grouping"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<CtBarGrouping>,

    #[serde(rename(serialize = "varyColors", deserialize = "varyColors"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vary_colors: Option<CtBoolean>,

    #[serde(rename(serialize = "ser", deserialize = "ser"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ser: Option<Vec<CtBarSer>>,

    #[serde(rename(serialize = "dLbls", deserialize = "dLbls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_lbls: Option<Box<CtDLbls>>,
}
