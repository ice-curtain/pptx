use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionListModify;

use crate::schemas::presentation::main::CtShape;

use crate::schemas::presentation::main::CtGraphicalObjectFrame;

use crate::schemas::presentation::main::CtGroupShapeNonVisual;

use crate::schemas::presentation::main::CtConnector;

use crate::schemas::presentation::main::CtRel;

use crate::schemas::presentation::main::CtPicture;

use crate::schemas::drawing::main::CtGroupShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_GroupShape
 */
#[derive(Serialize, Debug)]
pub struct CtGroupShape {
    #[serde(rename(serialize = "p:nvGrpSpPr", deserialize = "p:nvGrpSpPr"))]
    pub nvGrpSpPr: CtGroupShapeNonVisual,

    #[serde(rename(serialize = "p:grpSpPr", deserialize = "p:grpSpPr"))]
    pub grpSpPr: CtGroupShapeProperties,

    #[serde(rename = "$value")]
    pub items: Vec<ShapeChoice>,

    #[serde(rename(serialize = "p:extLst", deserialize = "p:extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extLst: Option<CtExtensionListModify>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShapeChoice {
    #[serde(rename(serialize = "p:sp", deserialize = "sp"))]
    Shape(CtShape),

    #[serde(rename(serialize = "p:grpSp", deserialize = "grpSp"))]
    GroupShape(CtGroupShape),

    #[serde(rename(serialize = "p:graphicFrame", deserialize = "graphicFrame"))]
    GraphicFrame(CtGroupShape),

    #[serde(rename(serialize = "p:cxnSp", deserialize = "cxnSp"))]
    CxnSp(CtConnector),

    #[serde(rename(serialize = "p:pic", deserialize = "pic"))]
    Pic(CtPicture),

    #[serde(rename(serialize = "p:contentPart", deserialize = "contentPart"))]
    ContentPart(CtRel),
}


impl<'de> serde::Deserialize<'de> for CtGroupShape {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error> where __D: serde::Deserializer<'de>, {
        #[allow(non_camel_case_types)]
        enum __Field { __field0, __field1, __field2, __field3, __ignore }
        struct __FieldVisitor;
        impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter) -> serde::__private::fmt::Result { serde::__private::Formatter::write_str(__formatter, "field identifier") }

            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E> where __E: serde::de::Error, {
                match __value {
                    "nvGrpSpPr" => serde::__private::Ok(__Field::__field0),
                    "grpSpPr" => serde::__private::Ok(__Field::__field1),
                    "$value" => serde::__private::Ok(__Field::__field2),
                    "extLst" => serde::__private::Ok(__Field::__field3),
                    _ => { serde::__private::Ok(__Field::__ignore) }
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error> where __D: serde::Deserializer<'de>, { serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor) }
        }
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<CtGroupShape>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = CtGroupShape;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter) -> serde::__private::fmt::Result { serde::__private::Formatter::write_str(__formatter, "struct CtGroupShape") }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error> where __A: serde::de::MapAccess<'de>, {
                let mut __field0: serde::__private::Option<CtGroupShapeNonVisual> = serde::__private::None;
                let mut __field1: serde::__private::Option<CtGroupShapeProperties> = serde::__private::None;
                let mut __field2: serde::__private::Option<Vec<ShapeChoice>> = serde::__private::None;
                let mut __field3: serde::__private::Option<Option<CtExtensionListModify>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) { return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("p:nvGrpSpPr")); }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<CtGroupShapeNonVisual>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) { return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("p:grpSpPr")); }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<CtGroupShapeProperties>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) { return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("$value")); }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<Vec<ShapeChoice>>(&mut __map)?);
                        }
                        __Field::__field3 => {
                            if serde::__private::Option::is_some(&__field3) { return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("p:extLst")); }
                            __field3 = serde::__private::Some(serde::de::MapAccess::next_value::<Option<CtExtensionListModify>>(&mut __map)?);
                        }
                        _ => { let _ = serde :: de :: MapAccess :: next_value :: < serde :: de :: IgnoredAny > (& mut __map )?; }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde :: __private :: de :: missing_field ("p:nvGrpSpPr" )?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde :: __private :: de :: missing_field ("p:grpSpPr" )?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde :: __private :: de :: missing_field ("$value" )?,
                };
                let __field3 = match __field3 {
                    serde::__private::Some(__field3) => __field3,
                    serde::__private::None => serde :: __private :: de :: missing_field ("p:extLst" )?,
                };
                serde::__private::Ok(CtGroupShape {
                    nvGrpSpPr: __field0,
                    grpSpPr: __field1,
                    items: __field2,
                    extLst: __field3,
                })
            }
        }
        const FIELDS: &'static [&'static str] = &["p:nvGrpSpPr", "p:grpSpPr", "$value", "p:extLst"];
        serde::Deserializer::deserialize_struct(__deserializer, "CtGroupShape", FIELDS, __Visitor { marker: serde::__private::PhantomData::<CtGroupShape>, lifetime: serde::__private::PhantomData })
    }
}


//
// # [serde(rename(serialize = "p:sp", deserialize = "sp"))]
// # [serde(skip_serializing_if = "Option::is_none")]
// pub sp: Option<Vec<CtShape> >,
//
// # [serde(rename(serialize = "p:grpSp", deserialize = "grpSp"))]
// # [serde(skip_serializing_if = "Option::is_none")]
// pub grp_sp: Option<Vec<CtGroupShape> >,
//
// # [serde(rename(serialize = "p:graphicFrame", deserialize = "graphicFrame"))]
// # [serde(skip_serializing_if = "Option::is_none")]
// pub graphic_frame: Option<Vec<CtGraphicalObjectFrame> >,
//
// # [serde(rename(serialize = "p:cxnSp", deserialize = "cxnSp"))]
// # [serde(skip_serializing_if = "Option::is_none")]
// pub cxn_sp: Option<Vec<CtConnector> >,
//
// # [serde(rename(serialize = "p:pic", deserialize = "pic"))]
// # [serde(skip_serializing_if = "Option::is_none")]
// pub pic: Option<Vec<CtPicture> >,
//
// # [serde(rename(serialize = "p:contentPart", deserialize = "contentPart"))]
// # [serde(skip_serializing_if = "Option::is_none")]
// pub content_part: Option<Vec<CtRel> >,