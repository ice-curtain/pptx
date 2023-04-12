use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtLvl;

/**
 * @author : zhilong.zhou
 * @description : CT_MultiLvlStrData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtMultiLvlStrData {
    #[serde(rename(serialize = "ptCount", deserialize = "ptCount"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt_count: Option<CtUnsignedInt>,

    #[serde(rename(serialize = "lvl", deserialize = "lvl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvl: Option<Vec<CtLvl>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
