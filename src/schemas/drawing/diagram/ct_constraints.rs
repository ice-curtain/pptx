use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtConstraint;

/**
 * @author : zhilong.zhou
 * @description : CT_Constraints
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtConstraints {
    #[serde(rename(serialize = "constr", deserialize = "constr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constr: Option<Vec<CtConstraint>>,
}
