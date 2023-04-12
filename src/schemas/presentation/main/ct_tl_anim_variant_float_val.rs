use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLAnimVariantFloatVal
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlAnimVariantFloatVal {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
