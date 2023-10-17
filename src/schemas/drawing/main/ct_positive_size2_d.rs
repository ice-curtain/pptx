use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_PositiveSize2D
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CtPositiveSize2D {
    ///min = 0,max = 27273042316900
    #[serde(rename = "@cx")]
    pub cx_attr: u64,

    ///min = 0,max = 27273042316900
    #[serde(rename = "@cy")]
    pub cy_attr: u64,
}
