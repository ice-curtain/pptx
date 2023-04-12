use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Marker
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtMarker {
    #[serde(rename(serialize = "x", deserialize = "x"))]
    pub x: String,

    #[serde(rename(serialize = "y", deserialize = "y"))]
    pub y: String,
}
