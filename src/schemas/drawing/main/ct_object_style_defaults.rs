use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtDefaultShapeDefinition;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_ObjectStyleDefaults
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtObjectStyleDefaults {
    #[serde(rename(serialize = "a:spDef", deserialize = "spDef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_def: Option<Box<CtDefaultShapeDefinition>>,

    #[serde(rename(serialize = "a:lnDef", deserialize = "lnDef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln_def: Option<Box<CtDefaultShapeDefinition>>,

    #[serde(rename(serialize = "a:txDef", deserialize = "txDef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_def: Option<Box<CtDefaultShapeDefinition>>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
