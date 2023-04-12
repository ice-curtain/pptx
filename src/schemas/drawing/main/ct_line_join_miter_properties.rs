use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_LineJoinMiterProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLineJoinMiterProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lim")]
    pub lim_attr: Option<String>,
}
