use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtTableCell;

/**
 * @author : zhilong.zhou
 * @description : CT_TableRow
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTableRow {
    #[serde(rename = "@h")]
    pub h_attr: String,

    #[serde(rename(serialize = "a:tc", deserialize = "tc"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<Vec<CtTableCell>>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
