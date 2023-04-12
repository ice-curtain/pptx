use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlTimeCondition;

use crate::schemas::presentation::main::CtTlIterateData;

use crate::schemas::presentation::main::CtTimeNodeList;

use crate::schemas::presentation::main::CtTlTimeConditionList;

/**
 * @author : zhilong.zhou
 * @description : CT_TLCommonTimeNodeData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlCommonTimeNodeData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@id")]
    pub id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@presetID")]
    pub preset_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@presetClass")]
    pub preset_class_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@presetSubtype")]
    pub preset_subtype_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dur")]
    pub dur_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@repeatCount")]
    pub repeat_count_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@repeatDur")]
    pub repeat_dur_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@spd")]
    pub spd_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@accel")]
    pub accel_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@decel")]
    pub decel_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@autoRev")]
    pub auto_rev_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@restart")]
    pub restart_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fill")]
    pub fill_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@syncBehavior")]
    pub sync_behavior_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@tmFilter")]
    pub tm_filter_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@evtFilter")]
    pub evt_filter_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@display")]
    pub display_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@masterRel")]
    pub master_rel_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bldLvl")]
    pub bld_lvl_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@grpId")]
    pub grp_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@afterEffect")]
    pub after_effect_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@nodeType")]
    pub node_type_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@nodePh")]
    pub node_ph_attr: Option<String>,

    #[serde(rename(serialize = "p:stCondLst", deserialize = "stCondLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub st_cond_lst: Option<CtTlTimeConditionList>,

    #[serde(rename(serialize = "p:endCondLst", deserialize = "endCondLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_cond_lst: Option<CtTlTimeConditionList>,

    #[serde(rename(serialize = "p:endSync", deserialize = "endSync"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_sync: Option<CtTlTimeCondition>,

    #[serde(rename(serialize = "p:iterate", deserialize = "iterate"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterate: Option<CtTlIterateData>,

    #[serde(rename(serialize = "p:childTnLst", deserialize = "childTnLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_tn_lst: Option<CtTimeNodeList>,

    #[serde(rename(serialize = "p:subTnLst", deserialize = "subTnLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_tn_lst: Option<CtTimeNodeList>,
}
