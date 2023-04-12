use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TileInfoProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTileInfoProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@tx")]
    pub tx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@ty")]
    pub ty_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sx")]
    pub sx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sy")]
    pub sy_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@flip")]
    pub flip_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@algn")]
    pub algn_attr: Option<String>,
}
