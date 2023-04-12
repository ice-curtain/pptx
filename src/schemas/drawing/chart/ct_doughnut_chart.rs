use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtHoleSize;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtFirstSliceAng;

use crate::schemas::drawing::chart::CtDLbls;

use crate::schemas::drawing::chart::CtPieSer;

/**
 * @author : zhilong.zhou
 * @description : CT_DoughnutChart
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDoughnutChart {
    #[serde(rename(serialize = "firstSliceAng", deserialize = "firstSliceAng"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_slice_ang: Option<CtFirstSliceAng>,

    #[serde(rename(serialize = "holeSize", deserialize = "holeSize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hole_size: Option<CtHoleSize>,

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
