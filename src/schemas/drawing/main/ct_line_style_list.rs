use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtLineProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_LineStyleList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLineStyleList {
    #[serde(rename(serialize = "a:ln", deserialize = "ln"))]
    pub ln: Vec<CtLineProperties>,
}
