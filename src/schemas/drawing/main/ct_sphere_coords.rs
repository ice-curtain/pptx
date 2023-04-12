use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_SphereCoords
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSphereCoords {
    #[serde(rename = "@lat")]
    pub lat_attr: String,

    #[serde(rename = "@lon")]
    pub lon_attr: String,

    #[serde(rename = "@rev")]
    pub rev_attr: String,
}
