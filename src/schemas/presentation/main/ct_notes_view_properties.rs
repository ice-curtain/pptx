use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

use crate::schemas::presentation::main::CtCommonSlideViewProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_NotesViewProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNotesViewProperties {
    #[serde(rename(serialize = "p:cSldViewPr", deserialize = "cSldViewPr"))]
    pub c_sld_view_pr: CtCommonSlideViewProperties,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
