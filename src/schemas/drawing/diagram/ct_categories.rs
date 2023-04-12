use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtCategory;

/**
 * @author : zhilong.zhou
 * @description : CT_Categories
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCategories {
    #[serde(rename(serialize = "cat", deserialize = "cat"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<CtCategory>>,
}
