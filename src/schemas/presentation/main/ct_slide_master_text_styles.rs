use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTextListStyle;

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_SlideMasterTextStyles
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSlideMasterTextStyles {
    #[serde(rename(serialize = "p:titleStyle", deserialize = "titleStyle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_style: Option<CtTextListStyle>,

    #[serde(rename(serialize = "p:bodyStyle", deserialize = "bodyStyle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_style: Option<CtTextListStyle>,

    #[serde(rename(serialize = "p:otherStyle", deserialize = "otherStyle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_style: Option<CtTextListStyle>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
