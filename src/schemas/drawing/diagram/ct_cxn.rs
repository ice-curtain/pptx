use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_Cxn
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCxn {
    #[serde(rename = "@modelId")]
    pub model_id_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type_attr: Option<String>,

    #[serde(rename = "@srcId")]
    pub src_id_attr: String,

    #[serde(rename = "@destId")]
    pub dest_id_attr: String,

    #[serde(rename = "@srcOrd")]
    pub src_ord_attr: String,

    #[serde(rename = "@destOrd")]
    pub dest_ord_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@parTransId")]
    pub par_trans_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sibTransId")]
    pub sib_trans_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@presId")]
    pub pres_id_attr: Option<String>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
