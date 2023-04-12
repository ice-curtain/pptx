use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Parameter
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtParameter {
    #[serde(rename = "@type")]
    pub r#type_attr: String,

    #[serde(rename = "@val")]
    pub val_attr: String,
}
