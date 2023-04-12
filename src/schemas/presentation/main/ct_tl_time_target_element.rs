use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlSubShapeId;

use crate::schemas::presentation::main::CtTlShapeTargetElement;

use crate::schemas::drawing::main::CtEmbeddedWavAudioFile;

use crate::schemas::presentation::main::CtEmpty;

/**
 * @author : zhilong.zhou
 * @description : CT_TLTimeTargetElement
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTimeTargetElement {
    #[serde(rename(serialize = "p:sldTgt", deserialize = "sldTgt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_tgt: Option<CtEmpty>,

    #[serde(rename(serialize = "p:sndTgt", deserialize = "sndTgt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snd_tgt: Option<CtEmbeddedWavAudioFile>,

    #[serde(rename(serialize = "p:spTgt", deserialize = "spTgt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_tgt: Option<CtTlShapeTargetElement>,

    #[serde(rename(serialize = "p:inkTgt", deserialize = "inkTgt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ink_tgt: Option<CtTlSubShapeId>,
}
