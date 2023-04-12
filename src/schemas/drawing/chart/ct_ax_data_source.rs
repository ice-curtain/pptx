use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtNumData;

use crate::schemas::drawing::chart::CtNumRef;

use crate::schemas::drawing::chart::CtStrRef;

use crate::schemas::drawing::chart::CtMultiLvlStrRef;

use crate::schemas::drawing::chart::CtStrData;

/**
 * @author : zhilong.zhou
 * @description : CT_AxDataSource
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAxDataSource {
    #[serde(rename(serialize = "multiLvlStrRef", deserialize = "multiLvlStrRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_lvl_str_ref: Option<CtMultiLvlStrRef>,

    #[serde(rename(serialize = "numRef", deserialize = "numRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ref: Option<CtNumRef>,

    #[serde(rename(serialize = "numLit", deserialize = "numLit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_lit: Option<CtNumData>,

    #[serde(rename(serialize = "strRef", deserialize = "strRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str_ref: Option<CtStrRef>,

    #[serde(rename(serialize = "strLit", deserialize = "strLit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str_lit: Option<CtStrData>,
}
