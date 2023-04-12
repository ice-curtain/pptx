use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_ShowInfoKiosk
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtShowInfoKiosk {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@restart")]
    pub restart_attr: Option<String>,
}
