use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtBubbleScale;

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtBubbleSer;

use crate::schemas::drawing::chart::CtDLbls;

use crate::schemas::drawing::chart::CtSizeRepresents;

use crate::schemas::drawing::chart::CtUnsignedInt;

/**
 * @author : zhilong.zhou
 * @description : CT_BubbleChart
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBubbleChart {
    #[serde(rename(serialize = "varyColors", deserialize = "varyColors"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vary_colors: Option<CtBoolean>,

    #[serde(rename(serialize = "ser", deserialize = "ser"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ser: Option<Vec<CtBubbleSer>>,

    #[serde(rename(serialize = "dLbls", deserialize = "dLbls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_lbls: Option<Box<CtDLbls>>,

    #[serde(rename(serialize = "bubble3D", deserialize = "bubble3D"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bubble3_d: Option<CtBoolean>,

    #[serde(rename(serialize = "bubbleScale", deserialize = "bubbleScale"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bubble_scale: Option<CtBubbleScale>,

    #[serde(rename(serialize = "showNegBubbles", deserialize = "showNegBubbles"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_neg_bubbles: Option<CtBoolean>,

    #[serde(rename(serialize = "sizeRepresents", deserialize = "sizeRepresents"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_represents: Option<CtSizeRepresents>,

    #[serde(rename(serialize = "axId", deserialize = "axId"))]
    pub ax_id: Vec<CtUnsignedInt>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
