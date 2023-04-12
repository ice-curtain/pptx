use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_ReflectionEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtReflectionEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@blurRad")]
    pub blur_rad_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@stA")]
    pub st_a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@stPos")]
    pub st_pos_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@endA")]
    pub end_a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@endPos")]
    pub end_pos_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dist")]
    pub dist_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dir")]
    pub dir_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fadeDir")]
    pub fade_dir_attr: Option<String>,

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
    #[serde(rename = "@algn")]
    pub algn_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rotWithShape")]
    pub rot_with_shape_attr: Option<String>,
}
