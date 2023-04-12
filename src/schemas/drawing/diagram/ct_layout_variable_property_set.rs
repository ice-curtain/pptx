use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtAnimLvl;

use crate::schemas::drawing::diagram::CtOrgChart;

use crate::schemas::drawing::diagram::CtResizeHandles;

use crate::schemas::drawing::diagram::CtBulletEnabled;

use crate::schemas::drawing::diagram::CtChildPref;

use crate::schemas::drawing::diagram::CtAnimOne;

use crate::schemas::drawing::diagram::CtDirection;

use crate::schemas::drawing::diagram::CtHierBranchStyle;

use crate::schemas::drawing::diagram::CtChildMax;

/**
 * @author : zhilong.zhou
 * @description : CT_LayoutVariablePropertySet
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLayoutVariablePropertySet {
    #[serde(rename(serialize = "orgChart", deserialize = "orgChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_chart: Option<CtOrgChart>,

    #[serde(rename(serialize = "chMax", deserialize = "chMax"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_max: Option<CtChildMax>,

    #[serde(rename(serialize = "chPref", deserialize = "chPref"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_pref: Option<CtChildPref>,

    #[serde(rename(serialize = "bulletEnabled", deserialize = "bulletEnabled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet_enabled: Option<CtBulletEnabled>,

    #[serde(rename(serialize = "dir", deserialize = "dir"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<CtDirection>,

    #[serde(rename(serialize = "hierBranch", deserialize = "hierBranch"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hier_branch: Option<CtHierBranchStyle>,

    #[serde(rename(serialize = "animOne", deserialize = "animOne"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anim_one: Option<CtAnimOne>,

    #[serde(rename(serialize = "animLvl", deserialize = "animLvl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anim_lvl: Option<CtAnimLvl>,

    #[serde(rename(serialize = "resizeHandles", deserialize = "resizeHandles"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_handles: Option<CtResizeHandles>,
}
