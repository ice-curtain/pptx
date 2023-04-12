use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_CustomerData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCustomerData {
    #[serde(rename = "@r:id")]
    pub r_id_attr: String,
}
