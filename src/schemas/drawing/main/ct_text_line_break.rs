use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTextCharacterProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_TextLineBreak
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextLineBreak {
    #[serde(rename(serialize = "a:rPr", deserialize = "rPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_pr: Option<Box<CtTextCharacterProperties>>,
}
