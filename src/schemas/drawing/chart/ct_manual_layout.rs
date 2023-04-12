use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtDouble;

use crate::schemas::drawing::chart::CtLayoutTarget;

use crate::schemas::drawing::chart::CtLayoutMode;

use crate::schemas::drawing::chart::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_ManualLayout
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtManualLayout {
    #[serde(rename(serialize = "layoutTarget", deserialize = "layoutTarget"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_target: Option<CtLayoutTarget>,

    #[serde(rename(serialize = "xMode", deserialize = "xMode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_mode: Option<CtLayoutMode>,

    #[serde(rename(serialize = "yMode", deserialize = "yMode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_mode: Option<CtLayoutMode>,

    #[serde(rename(serialize = "wMode", deserialize = "wMode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w_mode: Option<CtLayoutMode>,

    #[serde(rename(serialize = "hMode", deserialize = "hMode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_mode: Option<CtLayoutMode>,

    #[serde(rename(serialize = "x", deserialize = "x"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<CtDouble>,

    #[serde(rename(serialize = "y", deserialize = "y"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<CtDouble>,

    #[serde(rename(serialize = "w", deserialize = "w"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<CtDouble>,

    #[serde(rename(serialize = "h", deserialize = "h"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<CtDouble>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
