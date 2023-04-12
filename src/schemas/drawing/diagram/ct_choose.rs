use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtOtherwise;

use crate::schemas::drawing::diagram::CtWhen;

/**
 * @author : zhilong.zhou
 * @description : CT_Choose
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtChoose {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@name")]
    pub name_attr: Option<String>,

    #[serde(rename(serialize = "if", deserialize = "if"))]
    pub r#if: Vec<CtWhen>,

    #[serde(rename(serialize = "else", deserialize = "else"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#else: Option<Vec<CtOtherwise>>,
}
