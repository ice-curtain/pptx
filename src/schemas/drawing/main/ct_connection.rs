use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Connection
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtConnection {
    #[serde(rename = "@id")]
    pub id_attr: String,

    #[serde(rename = "@idx")]
    pub idx_attr: String,
}
