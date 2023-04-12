use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_LinearShadeProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLinearShadeProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@ang")]
    pub ang_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@scaled")]
    pub scaled_attr: Option<String>,
}
