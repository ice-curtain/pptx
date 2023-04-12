use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_WheelTransition
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtWheelTransition {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@spokes")]
    pub spokes_attr: Option<String>,
}
