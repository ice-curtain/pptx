use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_SplitTransition
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSplitTransition {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@orient")]
    pub orient_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dir")]
    pub dir_attr: Option<String>,
}
