use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TagsData
 */
#[derive(Serialize, Debug)]
pub struct CtTagsData {
    #[serde(rename = "@r:id")]
    pub r_id_attr: String,
}


impl<'de> serde::Deserialize<'de> for CtTagsData {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error> where __D: serde::Deserializer<'de>, {
        #[allow(non_camel_case_types)]
        enum __Field { __field0, __ignore }
        struct __FieldVisitor;
        impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter) -> serde::__private::fmt::Result { serde::__private::Formatter::write_str(__formatter, "field identifier") }

            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E> where __E: serde::de::Error, {
                match __value {
                    "@r:id" => serde::__private::Ok(__Field::__field0),
                    _ => { serde::__private::Ok(__Field::__ignore) }
                }
            }

        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error> where __D: serde::Deserializer<'de>, { serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor) }
        }
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<CtTagsData>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = CtTagsData;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter) -> serde::__private::fmt::Result { serde::__private::Formatter::write_str(__formatter, "struct CtTagsData") }

            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error> where __A: serde::de::MapAccess<'de>, {
                let mut __field0: serde::__private::Option<String> = serde::__private::None;
                let __field0_key = serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                __field0 = serde::__private::Some(serde :: de :: MapAccess :: next_value :: < String > (& mut __map )?);

                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde :: __private :: de :: missing_field ("@r:id" )?,
                };
                serde::__private::Ok(CtTagsData { r_id_attr: __field0 })
            }
        }
        const FIELDS: &'static [&'static str] = &["@r:id"];
        serde::Deserializer::deserialize_struct(__deserializer, "CtTagsData", FIELDS, __Visitor { marker: serde::__private::PhantomData::<CtTagsData>, lifetime: serde::__private::PhantomData })
    }
}
