use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLAnimVariantBooleanVal
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlAnimVariantBooleanVal {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
