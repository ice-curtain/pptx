use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtScatterStyle;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtDLbls;

use crate::schemas::drawing::chart::CtScatterSer;

use crate::schemas::drawing::chart::CtUnsignedInt;

/**
 * @author : zhilong.zhou
 * @description : CT_ScatterChart
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtScatterChart {
    #[serde(rename(serialize = "scatterStyle", deserialize = "scatterStyle"))]
    pub scatter_style: CtScatterStyle,

    #[serde(rename(serialize = "varyColors", deserialize = "varyColors"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vary_colors: Option<CtBoolean>,

    #[serde(rename(serialize = "ser", deserialize = "ser"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ser: Option<Vec<CtScatterSer>>,

    #[serde(rename(serialize = "dLbls", deserialize = "dLbls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_lbls: Option<Box<CtDLbls>>,

    #[serde(rename(serialize = "axId", deserialize = "axId"))]
    pub ax_id: Vec<CtUnsignedInt>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
