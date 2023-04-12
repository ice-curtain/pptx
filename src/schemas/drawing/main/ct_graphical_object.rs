use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtGraphicalObjectData;

/**
 * @author : zhilong.zhou
 * @description : CT_GraphicalObject
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "a:graphic", deserialize = "graphic"))]

pub struct CtGraphicalObject {
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

    #[serde(rename(serialize = "a:graphicData", deserialize = "graphicData"))]
    pub graphic_data: Vec<CtGraphicalObjectData>,
}
