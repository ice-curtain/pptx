use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename(serialize = "cp:coreProperties", deserialize = "coreProperties"))]
pub struct Core {
    #[serde(rename = "@xmlns:cp")]
    pub cp: String,

    #[serde(rename = "@xmlns:dc")]
    pub dc: String,

    #[serde(rename = "@xmlns:dcterms")]
    pub dcterms: String,

    #[serde(rename = "@xmlns:dcmitype")]
    pub dcmitype: Option<String>,

    #[serde(rename = "@xmlns:xsi")]
    pub xsi: String,

    #[serde(rename(deserialize = "title", serialize = "dc:title"))]
    pub title: String,

    #[serde(rename(deserialize = "creator", serialize = "dc:creator"))]
    pub creator: String,

    #[serde(rename(deserialize = "lastModifiedBy", serialize = "cp:lastModifiedBy"))]
    pub last_modified_by: String,

    #[serde(rename(deserialize = "revision", serialize = "cp:revision"))]
    pub revision: String,

    #[serde(rename(deserialize = "created", serialize = "dcterms:created"))]
    pub created: DcTerms,

    #[serde(rename(deserialize = "modified", serialize = "dcterms:modified"))]
    pub modified: DcTerms,

}

#[derive(Debug, Serialize)]
pub struct DcTerms {
    #[serde(rename = "@xsi:type")]
    pub created_type: Option<String>,

    #[serde(rename = "$value")]
    body: String,
}

    impl<'de> serde::Deserialize<'de> for DcTerms {
        fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error> where __D: serde::Deserializer<'de>, {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __field1, __ignore }
            struct __FieldVisitor;
            impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut serde::__private::Formatter) -> serde::__private::fmt::Result { serde::__private::Formatter::write_str(__formatter, "field identifier") }

                fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E> where __E: serde::de::Error, {
                    match __value {
                        "@xsi:type" => serde::__private::Ok(__Field::__field0),
                        "$value" => serde::__private::Ok(__Field::__field1),
                        _ => { serde::__private::Ok(__Field::__ignore) }
                    }
                }

            }
            impl<'de> serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error> where __D: serde::Deserializer<'de>, { serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor) }
            }
            struct __Visitor<'de> {
                marker: serde::__private::PhantomData<DcTerms>,
                lifetime: serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DcTerms;
                fn expecting(&self, __formatter: &mut serde::__private::Formatter) -> serde::__private::fmt::Result { serde::__private::Formatter::write_str(__formatter, "struct DcTerms") }

                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error> where __A: serde::de::MapAccess<'de>, {
                    let field0_key = serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                    let mut __field0 = serde::__private::Some(serde :: de :: MapAccess :: next_value :: < Option<String> > (& mut __map )?);

                    let field1_key = serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                    let mut __field1: serde::__private::Option<String> =  serde::__private::Some(serde :: de :: MapAccess :: next_value :: < String > (& mut __map )?);

                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde :: __private :: de :: missing_field ("@xsi:type" )?,
                    };
                    let __field1 = match __field1 {
                        serde::__private::Some(__field1) => __field1,
                        serde::__private::None => serde :: __private :: de :: missing_field ("$value" )?,
                    };
                    serde::__private::Ok(DcTerms { created_type: __field0, body: __field1 })
                }
            }
            const FIELDS: &'static [&'static str] = &["@xsi:type", "$value"];
            serde::Deserializer::deserialize_struct(__deserializer, "DcTerms", FIELDS, __Visitor { marker: serde::__private::PhantomData::<DcTerms>, lifetime: serde::__private::PhantomData })
        }
    }




