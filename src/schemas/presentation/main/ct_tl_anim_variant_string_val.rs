use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLAnimVariantStringVal
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlAnimVariantStringVal {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
