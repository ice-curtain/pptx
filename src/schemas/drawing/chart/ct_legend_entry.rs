use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::main::CtTextBody;

/**
 * @author : zhilong.zhou
 * @description : CT_LegendEntry
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLegendEntry {
    #[serde(rename(serialize = "idx", deserialize = "idx"))]
    pub idx: CtUnsignedInt,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "delete", deserialize = "delete"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<CtBoolean>,

    #[serde(rename(serialize = "txPr", deserialize = "txPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_pr: Option<Box<CtTextBody>>,
}
