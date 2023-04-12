use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtension;

/**
 * @author : zhilong.zhou
 * @description : CT_OfficeArtExtensionList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtOfficeArtExtensionList {
    #[serde(rename(serialize = "a:ext", deserialize = "ext"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Vec<CtOfficeArtExtension>>,
}
