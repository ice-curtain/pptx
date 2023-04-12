use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtSlideRelationshipList;

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_CustomShow
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCustomShow {
    #[serde(rename = "@name")]
    pub name_attr: String,

    #[serde(rename = "@id")]
    pub id_attr: String,

    #[serde(rename(serialize = "p:sldLst", deserialize = "sldLst"))]
    pub sld_lst: CtSlideRelationshipList,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
