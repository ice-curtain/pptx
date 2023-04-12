use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtManualLayout;

use crate::schemas::drawing::chart::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_Layout
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLayout {
    #[serde(rename(serialize = "manualLayout", deserialize = "manualLayout"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_layout: Option<CtManualLayout>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
