use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlAnimateColorBehavior;

use crate::schemas::presentation::main::CtTlSetBehavior;

use crate::schemas::presentation::main::CtTlTimeNodeParallel;

use crate::schemas::presentation::main::CtTlTimeNodeSequence;

use crate::schemas::presentation::main::CtTlAnimateEffectBehavior;

use crate::schemas::presentation::main::CtTlAnimateMotionBehavior;

use crate::schemas::presentation::main::CtTlAnimateRotationBehavior;

use crate::schemas::presentation::main::CtTlAnimateScaleBehavior;

use crate::schemas::presentation::main::CtTlMediaNodeVideo;

use crate::schemas::presentation::main::CtTlCommandBehavior;

use crate::schemas::presentation::main::CtTlMediaNodeAudio;

use crate::schemas::presentation::main::CtTlAnimateBehavior;

use crate::schemas::presentation::main::CtTlTimeNodeExclusive;

/**
 * @author : zhilong.zhou
 * @description : CT_TimeNodeList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTimeNodeList {
    #[serde(rename(serialize = "p:par", deserialize = "par"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par: Option<Vec<CtTlTimeNodeParallel>>,

    #[serde(rename(serialize = "p:seq", deserialize = "seq"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<Vec<CtTlTimeNodeSequence>>,

    #[serde(rename(serialize = "p:excl", deserialize = "excl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excl: Option<Vec<CtTlTimeNodeExclusive>>,

    #[serde(rename(serialize = "p:anim", deserialize = "anim"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anim: Option<Vec<CtTlAnimateBehavior>>,

    #[serde(rename(serialize = "p:animClr", deserialize = "animClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anim_clr: Option<Vec<CtTlAnimateColorBehavior>>,

    #[serde(rename(serialize = "p:animEffect", deserialize = "animEffect"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anim_effect: Option<Vec<CtTlAnimateEffectBehavior>>,

    #[serde(rename(serialize = "p:animMotion", deserialize = "animMotion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anim_motion: Option<Vec<CtTlAnimateMotionBehavior>>,

    #[serde(rename(serialize = "p:animRot", deserialize = "animRot"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anim_rot: Option<Vec<CtTlAnimateRotationBehavior>>,

    #[serde(rename(serialize = "p:animScale", deserialize = "animScale"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anim_scale: Option<Vec<CtTlAnimateScaleBehavior>>,

    #[serde(rename(serialize = "p:cmd", deserialize = "cmd"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<CtTlCommandBehavior>>,

    #[serde(rename(serialize = "p:set", deserialize = "set"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<CtTlSetBehavior>>,

    #[serde(rename(serialize = "p:audio", deserialize = "audio"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<CtTlMediaNodeAudio>>,

    #[serde(rename(serialize = "p:video", deserialize = "video"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Vec<CtTlMediaNodeVideo>>,
}
