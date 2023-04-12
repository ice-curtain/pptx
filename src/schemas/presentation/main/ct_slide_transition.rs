use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionListModify;

use crate::schemas::presentation::main::CtEmpty;

use crate::schemas::presentation::main::CtOptionalBlackTransition;

use crate::schemas::presentation::main::CtTransitionSoundAction;

use crate::schemas::presentation::main::CtSplitTransition;

use crate::schemas::presentation::main::CtInOutTransition;

use crate::schemas::presentation::main::CtWheelTransition;

use crate::schemas::presentation::main::CtOrientationTransition;

use crate::schemas::presentation::main::CtCornerDirectionTransition;

use crate::schemas::presentation::main::CtSideDirectionTransition;

use crate::schemas::presentation::main::CtEightDirectionTransition;

/**
 * @author : zhilong.zhou
 * @description : CT_SlideTransition
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSlideTransition {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@spd")]
    pub spd_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@advClick")]
    pub adv_click_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@advTm")]
    pub adv_tm_attr: Option<String>,

    #[serde(rename(serialize = "p:sndAc", deserialize = "sndAc"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snd_ac: Option<CtTransitionSoundAction>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,

    #[serde(rename(serialize = "p:blinds", deserialize = "blinds"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blinds: Option<CtOrientationTransition>,

    #[serde(rename(serialize = "p:checker", deserialize = "checker"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checker: Option<CtOrientationTransition>,

    #[serde(rename(serialize = "p:circle", deserialize = "circle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle: Option<CtEmpty>,

    #[serde(rename(serialize = "p:dissolve", deserialize = "dissolve"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolve: Option<CtEmpty>,

    #[serde(rename(serialize = "p:comb", deserialize = "comb"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comb: Option<CtOrientationTransition>,

    #[serde(rename(serialize = "p:cover", deserialize = "cover"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<CtEightDirectionTransition>,

    #[serde(rename(serialize = "p:cut", deserialize = "cut"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cut: Option<CtOptionalBlackTransition>,

    #[serde(rename(serialize = "p:diamond", deserialize = "diamond"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diamond: Option<CtEmpty>,

    #[serde(rename(serialize = "p:fade", deserialize = "fade"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade: Option<CtOptionalBlackTransition>,

    #[serde(rename(serialize = "p:newsflash", deserialize = "newsflash"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newsflash: Option<CtEmpty>,

    #[serde(rename(serialize = "p:plus", deserialize = "plus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plus: Option<CtEmpty>,

    #[serde(rename(serialize = "p:pull", deserialize = "pull"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull: Option<CtEightDirectionTransition>,

    #[serde(rename(serialize = "p:push", deserialize = "push"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push: Option<CtSideDirectionTransition>,

    #[serde(rename(serialize = "p:random", deserialize = "random"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random: Option<CtEmpty>,

    #[serde(rename(serialize = "p:randomBar", deserialize = "randomBar"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_bar: Option<CtOrientationTransition>,

    #[serde(rename(serialize = "p:split", deserialize = "split"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<CtSplitTransition>,

    #[serde(rename(serialize = "p:strips", deserialize = "strips"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strips: Option<CtCornerDirectionTransition>,

    #[serde(rename(serialize = "p:wedge", deserialize = "wedge"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wedge: Option<CtEmpty>,

    #[serde(rename(serialize = "p:wheel", deserialize = "wheel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheel: Option<CtWheelTransition>,

    #[serde(rename(serialize = "p:wipe", deserialize = "wipe"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wipe: Option<CtSideDirectionTransition>,

    #[serde(rename(serialize = "p:zoom", deserialize = "zoom"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<CtInOutTransition>,
}
