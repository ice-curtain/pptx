use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtCustomerData;

use crate::schemas::presentation::main::CtTagsData;

/**
 * @author : zhilong.zhou
 * @description : CT_CustomerDataList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCustomerDataList {
    #[serde(rename(serialize = "p:custData", deserialize = "custData"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_data: Option<Vec<CtCustomerData>>,

    #[serde(rename(serialize = "p:tags", deserialize = "tags"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<CtTagsData>,
}
