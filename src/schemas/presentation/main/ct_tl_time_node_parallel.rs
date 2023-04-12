use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlCommonTimeNodeData;

/**
 * @author : zhilong.zhou
 * @description : CT_TLTimeNodeParallel
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTimeNodeParallel {
    #[serde(rename(serialize = "p:cTn", deserialize = "cTn"))]
    pub c_tn: CtTlCommonTimeNodeData,
}
