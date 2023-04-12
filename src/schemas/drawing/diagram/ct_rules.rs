use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtNumericRule;

/**
 * @author : zhilong.zhou
 * @description : CT_Rules
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtRules {
    #[serde(rename(serialize = "rule", deserialize = "rule"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<CtNumericRule>>,
}
