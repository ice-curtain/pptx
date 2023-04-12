use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtBandFmt;

/**
 * @author : zhilong.zhou
 * @description : CT_BandFmts
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBandFmts {
    #[serde(rename(serialize = "bandFmt", deserialize = "bandFmt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_fmt: Option<Vec<CtBandFmt>>,
}
