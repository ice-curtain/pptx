use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtParameter;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_Algorithm
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAlgorithm {
    #[serde(rename = "@type")]
    pub r#type_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rev")]
    pub rev_attr: Option<String>,

    #[serde(rename(serialize = "param", deserialize = "param"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<Vec<CtParameter>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
