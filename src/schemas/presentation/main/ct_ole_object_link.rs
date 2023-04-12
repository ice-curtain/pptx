use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_OleObjectLink
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtOleObjectLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@updateAutomatic")]
    pub update_automatic_attr: Option<String>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
