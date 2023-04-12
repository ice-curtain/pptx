use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Path2DArcTo
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPath2DArcTo {
    #[serde(rename = "@wR")]
    pub w_r_attr: String,

    #[serde(rename = "@hR")]
    pub h_r_attr: String,

    #[serde(rename = "@stAng")]
    pub st_ang_attr: String,

    #[serde(rename = "@swAng")]
    pub sw_ang_attr: String,
}
