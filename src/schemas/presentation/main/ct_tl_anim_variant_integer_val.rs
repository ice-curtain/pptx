use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLAnimVariantIntegerVal
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlAnimVariantIntegerVal {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
