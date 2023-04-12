use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTransitionStartSoundAction;

use crate::schemas::presentation::main::CtEmpty;

/**
 * @author : zhilong.zhou
 * @description : CT_TransitionSoundAction
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTransitionSoundAction {
    #[serde(rename(serialize = "p:stSnd", deserialize = "stSnd"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub st_snd: Option<CtTransitionStartSoundAction>,

    #[serde(rename(serialize = "p:endSnd", deserialize = "endSnd"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_snd: Option<CtEmpty>,
}
