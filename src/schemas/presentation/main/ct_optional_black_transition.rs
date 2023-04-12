use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_OptionalBlackTransition
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtOptionalBlackTransition {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@thruBlk")]
    pub thru_blk_attr: Option<String>,
}
