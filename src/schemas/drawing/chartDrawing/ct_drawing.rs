use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chartDrawing::CtRelSizeAnchor;

use crate::schemas::drawing::chartDrawing::CtAbsSizeAnchor;

/**
 * @author : zhilong.zhou
 * @description : CT_Drawing
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "userShapes", deserialize = "userShapes"))]

pub struct CtDrawing {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:p")]
    pub p_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:a")]
    pub a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:r")]
    pub r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:s")]
    pub s_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns")]
    pub default_namespace_attr: Option<String>,

    #[serde(rename(serialize = "relSizeAnchor", deserialize = "relSizeAnchor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel_size_anchor: Option<Vec<CtRelSizeAnchor>>,

    #[serde(rename(serialize = "absSizeAnchor", deserialize = "absSizeAnchor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abs_size_anchor: Option<Vec<CtAbsSizeAnchor>>,
}
