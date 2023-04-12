use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtSdCategory;

/**
 * @author : zhilong.zhou
 * @description : CT_SDCategories
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSdCategories {
    #[serde(rename(serialize = "cat", deserialize = "cat"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<CtSdCategory>>,
}
