use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtension;

/**
 * @author : zhilong.zhou
 * @description : CT_ExtensionListModify
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtExtensionListModify {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@mod")]
    pub mod_attr: Option<String>,

    #[serde(rename(serialize = "p:ext", deserialize = "ext"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Vec<CtExtension>>,
}
