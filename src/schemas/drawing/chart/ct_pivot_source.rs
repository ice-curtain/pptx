use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_PivotSource
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPivotSource {
    #[serde(rename(serialize = "name", deserialize = "name"))]
    pub name: String,

    #[serde(rename(serialize = "fmtId", deserialize = "fmtId"))]
    pub fmt_id: CtUnsignedInt,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<Vec<CtExtensionList>>,
}
