use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtCommonViewProperties;

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_NotesTextViewProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNotesTextViewProperties {
    #[serde(rename(serialize = "p:cViewPr", deserialize = "cViewPr"))]
    pub c_view_pr: CtCommonViewProperties,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
