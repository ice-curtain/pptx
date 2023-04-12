use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtTx;

use crate::schemas::drawing::chart::CtLayout;

/**
 * @author : zhilong.zhou
 * @description : CT_DLbl
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDLbl {
    #[serde(rename(serialize = "idx", deserialize = "idx"))]
    pub idx: CtUnsignedInt,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "delete", deserialize = "delete"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<CtBoolean>,

    #[serde(rename(serialize = "layout", deserialize = "layout"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<CtLayout>,

    #[serde(rename(serialize = "tx", deserialize = "tx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<Box<CtTx>>,
}
