use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtEmbeddedWavAudioFile;

/**
 * @author : zhilong.zhou
 * @description : CT_TransitionStartSoundAction
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTransitionStartSoundAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@loop")]
    pub loop_attr: Option<String>,

    #[serde(rename(serialize = "p:snd", deserialize = "snd"))]
    pub snd: CtEmbeddedWavAudioFile,
}
