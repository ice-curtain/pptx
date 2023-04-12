use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_SlideSyncProperties
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:sldSyncPr", deserialize = "sldSyncPr"))]

pub struct CtSlideSyncProperties {
    #[serde(rename = "@serverSldId")]
    pub server_sld_id_attr: String,

    #[serde(rename = "@serverSldModifiedTime")]
    pub server_sld_modified_time_attr: String,

    #[serde(rename = "@clientInsertedTime")]
    pub client_inserted_time_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:p")]
    pub p_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:a")]
    pub a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:r")]
    pub r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:s")]
    pub s_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns")]
    pub default_namespace_attr: Option<String>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
