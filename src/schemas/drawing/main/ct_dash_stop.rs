use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_DashStop
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDashStop {
    #[serde(rename = "@d")]
    pub d_attr: String,

    #[serde(rename = "@sp")]
    pub sp_attr: String,
}
