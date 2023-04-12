use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_ShowInfoBrowse
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtShowInfoBrowse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showScrollbar")]
    pub show_scrollbar_attr: Option<String>,
}
