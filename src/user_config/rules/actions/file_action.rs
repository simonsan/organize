use crate::{
    path::Expandable,
    user_config::rules::{
        actions::{ConflictOption, Sep},
        deserialize::deserialize_path,
    },
};
use serde::{
    de,
    de::{MapAccess, Unexpected, Visitor},
    export,
    export::{fmt, PhantomData},
    Deserialize, Deserializer, Serialize,
};
use std::{path::PathBuf, result, str::FromStr};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct FileAction {
    #[serde(deserialize_with = "deserialize_path")]
    pub to: PathBuf,
    #[serde(default)]
    pub if_exists: ConflictOption,
    #[serde(default)]
    pub sep: Sep,
}

impl From<PathBuf> for FileAction {
    fn from(path: PathBuf) -> Self {
        Self {
            to: path.expand_user().expand_vars(),
            if_exists: Default::default(),
            sep: Default::default(),
        }
    }
}

impl FromStr for FileAction {
    type Err = ();

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let path = s.parse::<PathBuf>().unwrap();
        Ok(Self::from(path))
    }
}

// must always use with #[serde(default)]
pub fn optional_string_or_struct<'de, T, D>(deserializer: D) -> result::Result<Option<T>, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = ()>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = ()>,
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut export::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(T::from_str(value).unwrap()))
        }

        fn visit_none<E>(self) -> result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> result::Result<Self::Value, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
        {
            let action = T::deserialize(deserializer);
            match action {
                Ok(object) => Ok(Some(object)),
                Err(e) => Err(e),
            }
        }

        fn visit_map<M>(self, map: M) -> result::Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    match deserializer.deserialize_any(StringOrStruct(PhantomData)) {
        Ok(d) => Ok(d),
        Err(e) => Err(e),
    }
}
