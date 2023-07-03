use crate::schemas::presentation::main::{
    CtExtensionList, CtNotesMasterIdListEntry, CtSlideIdListEntry, CtSlideLayoutIdListEntry,
    CtSlideMasterIdList, CtSlideMasterIdListEntry,
};
use serde::de::Visitor;
use serde::Deserializer;
use std::fmt;
use std::fmt::Formatter;
use std::marker::PhantomData;

impl<'de> serde::Deserialize<'de> for CtSlideLayoutIdListEntry {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __ignore,
        }
        struct __FieldVisitor;
        impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
            type Value = __Field;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "@id" => serde::__private::Ok(__Field::__field0),
                    "@r:id" => serde::__private::Ok(__Field::__field1),
                    "extLst" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<CtSlideLayoutIdListEntry>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = CtSlideLayoutIdListEntry;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    __formatter,
                    "struct CtSlideLayoutIdListEntry",
                )
            }

            #[inline]
            fn visit_map<__A>(
                self,
                mut __map: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                let mut __field0: Option<Option<u32>> = Some(
                    serde::de::MapAccess::next_value::<Option<u32>>(&mut __map)?,
                );

                serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                let mut __field1: Option<String> =
                    Some(serde::de::MapAccess::next_value::<String>(&mut __map)?);

                let field2_key = serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                let mut __field2: serde::__private::Option<Option<CtExtensionList>> =
                    if field2_key.is_some() {
                        Some(serde::de::MapAccess::next_value::<Option<CtExtensionList>>(
                            &mut __map,
                        )?)
                    } else {
                        None
                    };

                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("@id")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("@r:id")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("extLst")?,
                };
                serde::__private::Ok(CtSlideLayoutIdListEntry {
                    id_attr: __field0,
                    r_id_attr: __field1,
                    ext_lst: __field2,
                })
            }
        }
        const FIELDS: &'static [&'static str] = &["@id", "@r:id", "extLst"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "CtSlideLayoutIdListEntry",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<CtSlideLayoutIdListEntry>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}

impl<'de> serde::Deserialize<'de> for CtSlideIdListEntry {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __ignore,
        }
        struct __FieldVisitor;
        impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
            type Value = __Field;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }

            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "@id" => serde::__private::Ok(__Field::__field0),
                    "@r:id" => serde::__private::Ok(__Field::__field1),
                    "extLst" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<CtSlideIdListEntry>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = CtSlideIdListEntry;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct CtSlideIdListEntry")
            }

            #[inline]
            fn visit_map<__A>(
                self,
                mut __map: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                let mut __field0: Option<Option<u32>> =
                    Some(serde::de::MapAccess::next_value::<Option<u32>>(&mut __map)?);

                serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                let mut __field1: Option<String> =
                    Some(serde::de::MapAccess::next_value::<String>(&mut __map)?);

                let field2_key = serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                let mut __field2: serde::__private::Option<Option<CtExtensionList>> =
                    if field2_key.is_some() {
                        Some(serde::de::MapAccess::next_value::<Option<CtExtensionList>>(
                            &mut __map,
                        )?)
                    } else {
                        None
                    };

                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("@id")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("@r:id")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("extLst")?,
                };
                serde::__private::Ok(CtSlideIdListEntry {
                    id_attr: __field0,
                    r_id_attr: __field1,
                    ext_lst: __field2,
                })
            }
        }
        const FIELDS: &'static [&'static str] = &["@id", "@r:id", "extLst"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "CtSlideIdListEntry",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<CtSlideIdListEntry>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}

impl<'de> serde::Deserialize<'de> for CtSlideMasterIdListEntry {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __ignore,
        }
        struct __FieldVisitor;
        impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
            type Value = __Field;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }

            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "@id" => serde::__private::Ok(__Field::__field0),
                    "@r:id" => serde::__private::Ok(__Field::__field1),
                    "extLst" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<CtSlideMasterIdListEntry>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = CtSlideMasterIdListEntry;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    __formatter,
                    "struct CtSlideMasterIdListEntry",
                )
            }

            #[inline]
            fn visit_map<__A>(
                self,
                mut __map: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let field0_key = serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                let mut __field0: serde::__private::Option<Option<String>> = Some(
                    serde::de::MapAccess::next_value::<Option<String>>(&mut __map)?,
                );

                let field1_key = serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                let mut __field1: serde::__private::Option<String> =
                    Some(serde::de::MapAccess::next_value::<String>(&mut __map)?);

                let field2_key = serde::de::MapAccess::next_key::<__Field>(&mut __map)?;
                let mut __field2: serde::__private::Option<Option<CtExtensionList>> =
                    if field2_key.is_some() {
                        Some(serde::de::MapAccess::next_value::<Option<CtExtensionList>>(
                            &mut __map,
                        )?)
                    } else {
                        None
                    };

                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("@id")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("@r:id")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("extLst")?,
                };
                serde::__private::Ok(CtSlideMasterIdListEntry {
                    id_attr: __field0,
                    r_id_attr: __field1,
                    ext_lst: __field2,
                })
            }
        }
        const FIELDS: &'static [&'static str] = &["@id", "@r:id", "extLst"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "CtSlideMasterIdListEntry",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<CtSlideMasterIdListEntry>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}

impl<'de> serde::Deserialize<'de> for CtNotesMasterIdListEntry {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        struct __FieldVisitor;
        impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
            type Value = __Field;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }

            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    // "@r:id" => serde::__private::Ok(__Field::__field0),
                    "@id" => serde::__private::Ok(__Field::__field0),
                    "extLst" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<CtNotesMasterIdListEntry>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = CtNotesMasterIdListEntry;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    __formatter,
                    "struct CtNotesMasterIdListEntry",
                )
            }

            #[inline]
            fn visit_map<__A>(
                self,
                mut __map: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<String> = serde::__private::None;
                let mut __field1: serde::__private::Option<Option<CtExtensionList>> =
                    serde::__private::None;
                while let serde::__private::Some(__key) =
                    serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("@r:id"),
                                );
                            }
                            __field0 =
                                serde::__private::Some(serde::de::MapAccess::next_value::<String>(
                                    &mut __map,
                                )?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("extLst"),
                                );
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                Option<CtExtensionList>,
                            >(
                                &mut __map
                            )?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(
                                &mut __map,
                            )?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("@r:id")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("extLst")?,
                };
                serde::__private::Ok(CtNotesMasterIdListEntry {
                    r_id_attr: __field0,
                    ext_lst: __field1,
                })
            }
        }
        const FIELDS: &'static [&'static str] = &["@r:id", "extLst"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "CtNotesMasterIdListEntry",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<CtNotesMasterIdListEntry>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
