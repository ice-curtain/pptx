use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_SideDirectionTransition
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSideDirectionTransition {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dir")]
    pub dir_attr: Option<String>,
}
