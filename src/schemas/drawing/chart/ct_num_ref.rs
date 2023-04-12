use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtNumData;

/**
 * @author : zhilong.zhou
 * @description : CT_NumRef
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNumRef {
    #[serde(rename(serialize = "f", deserialize = "f"))]
    pub f: String,

    #[serde(rename(serialize = "numCache", deserialize = "numCache"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_cache: Option<CtNumData>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
