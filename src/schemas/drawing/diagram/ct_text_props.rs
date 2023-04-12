use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtShape3D;

use crate::schemas::drawing::main::CtFlatText;

/**
 * @author : zhilong.zhou
 * @description : CT_TextProps
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextProps {
    #[serde(rename(serialize = "sp3d", deserialize = "sp3d"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp3d: Option<CtShape3D>,

    #[serde(rename(serialize = "flatTx", deserialize = "flatTx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_tx: Option<CtFlatText>,
}
