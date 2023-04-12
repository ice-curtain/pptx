use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlCommonTimeNodeData;

/**
 * @author : zhilong.zhou
 * @description : CT_TLTimeNodeExclusive
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTimeNodeExclusive {
    #[serde(rename(serialize = "p:cTn", deserialize = "cTn"))]
    pub c_tn: CtTlCommonTimeNodeData,
}
