use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TransformEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTransformEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sx")]
    pub sx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sy")]
    pub sy_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@kx")]
    pub kx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@ky")]
    pub ky_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@tx")]
    pub tx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@ty")]
    pub ty_attr: Option<String>,
}
