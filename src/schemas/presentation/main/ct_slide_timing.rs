use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionListModify;

use crate::schemas::presentation::main::CtBuildList;

use crate::schemas::presentation::main::CtTimeNodeList;

/**
 * @author : zhilong.zhou
 * @description : CT_SlideTiming
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSlideTiming {
    #[serde(rename(serialize = "p:tnLst", deserialize = "tnLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tn_lst: Option<CtTimeNodeList>,

    #[serde(rename(serialize = "p:bldLst", deserialize = "bldLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bld_lst: Option<CtBuildList>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,
}
