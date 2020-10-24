use super::deserialize::deserialize_path;
use crate::path::Expandable;
use serde::{
    de,
    de::{MapAccess, Visitor},
    export::PhantomData,
    Deserialize, Deserializer, Serialize,
};
use std::{fmt, ops::Deref, path::PathBuf, str::FromStr};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Folder {
    #[serde(deserialize_with = "deserialize_path")]
    pub path: PathBuf,
    #[serde(default)]
    pub options: Options,
}

impl From<PathBuf> for Folder {
    fn from(path: PathBuf) -> Self {
        Self {
            path: path.expand_user().expand_vars(),
            options: Default::default(),
        }
    }
}

impl FromStr for Folder {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = s.parse::<PathBuf>().unwrap();
        Ok(Self::from(path))
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct WrappedFolder(#[serde(deserialize_with = "string_or_struct")] Folder);

impl Deref for WrappedFolder {
    type Target = Folder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
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
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
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

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Options {
    /// defines whether or not subdirectories must be scanned
    #[serde(default)]
    pub recursive: bool,
    #[serde(default)]
    pub watch: bool,
    #[serde(default)]
    pub ignore: Vec<PathBuf>,
    #[serde(default)]
    pub hidden_files: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            recursive: false,
            watch: true,
            hidden_files: false,
            ignore: Vec::new(),
        }
    }
}
