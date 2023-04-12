use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtMultiLvlStrData;

use crate::schemas::drawing::chart::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_MultiLvlStrRef
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtMultiLvlStrRef {
    #[serde(rename(serialize = "f", deserialize = "f"))]
    pub f: String,

    #[serde(rename(serialize = "multiLvlStrCache", deserialize = "multiLvlStrCache"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_lvl_str_cache: Option<CtMultiLvlStrData>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
