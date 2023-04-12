use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionListModify;

use crate::schemas::drawing::main::CtPoint2D;

/**
 * @author : zhilong.zhou
 * @description : CT_Comment
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtComment {
    #[serde(rename = "@authorId")]
    pub author_id_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dt")]
    pub dt_attr: Option<String>,

    #[serde(rename = "@idx")]
    pub idx_attr: String,

    #[serde(rename(serialize = "p:pos", deserialize = "pos"))]
    pub pos: CtPoint2D,

    #[serde(rename(serialize = "p:text", deserialize = "text"))]
    pub text: String,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,
}
