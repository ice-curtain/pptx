use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtOrientation;

use crate::schemas::drawing::chart::CtDouble;

use crate::schemas::drawing::chart::CtLogBase;

use crate::schemas::drawing::chart::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_Scaling
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtScaling {
    #[serde(rename(serialize = "logBase", deserialize = "logBase"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_base: Option<CtLogBase>,

    #[serde(rename(serialize = "orientation", deserialize = "orientation"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<CtOrientation>,

    #[serde(rename(serialize = "max", deserialize = "max"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<CtDouble>,

    #[serde(rename(serialize = "min", deserialize = "min"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<CtDouble>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
