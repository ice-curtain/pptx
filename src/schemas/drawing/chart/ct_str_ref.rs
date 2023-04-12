use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtStrData;

/**
 * @author : zhilong.zhou
 * @description : CT_StrRef
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtStrRef {
    #[serde(rename(serialize = "f", deserialize = "f"))]
    pub f: String,

    #[serde(rename(serialize = "strCache", deserialize = "strCache"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str_cache: Option<CtStrData>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
