use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtNumVal;

use crate::schemas::drawing::chart::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_NumData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNumData {
    #[serde(rename(serialize = "formatCode", deserialize = "formatCode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_code: Option<String>,

    #[serde(rename(serialize = "ptCount", deserialize = "ptCount"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt_count: Option<CtUnsignedInt>,

    #[serde(rename(serialize = "pt", deserialize = "pt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<Vec<CtNumVal>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
