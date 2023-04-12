use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Ratio
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtRatio {
    #[serde(rename = "@n")]
    pub n_attr: String,

    #[serde(rename = "@d")]
    pub d_attr: String,
}
