use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtension;

/**
 * @author : zhilong.zhou
 * @description : CT_ExtensionList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtExtensionList {
    #[serde(rename(serialize = "ext", deserialize = "ext"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Vec<CtExtension>>,
}
