use std::{collections::HashMap, marker::PhantomData, str::FromStr};

use color_eyre::eyre;
use serde::{de, Deserialize, Serialize};

#[derive(Debug, derive_more::Display, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Backend {
    #[display(fmt = "carbon")]
    Carbon,
    #[default]
    #[display(fmt = "silicon")]
    Silicon,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "msg_type", content = "msg_body")]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    CopyrightReport {
        text: String,
    },
    WarningsDuringParsing(Vec<serde_json::Value>),
    InvalidArgsReport {
        tool: String,
        errors: Vec<DetailsError>,
    },
    AstConstructionResult {
        details: Details,
        status: String,
    },
    ProgramOutline {
        members: Vec<ProgramOutlineMember>,
    },
    ProgramDefinitions {
        definitions: Vec<ProgramDefinition>,
    },
    Statistics {
        domains: i64,
        fields: i64,
        functions: i64,
        methods: i64,
        predicates: i64,
    },
    ExceptionReport {
        message: String,
        stacktrace: Vec<String>,
    },
    ConfigurationConfirmation {
        text: String,
    },
    VerificationResult {
        details: Details,
        kind: String,
        status: String,
        verifier: Backend,
        #[serde(flatten)]
        extra: HashMap<String, serde_json::Value>,
    },
    BackendSubProcessReport {
        phase: String,
        pid: Option<i64>,
        process_exe: String,
        tool: Backend,
        #[serde(flatten)]
        extra: HashMap<String, serde_json::Value>,
    },
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    pub cached: Option<bool>,
    pub result: Option<DetailsResult>,
    pub time: i64,
    pub entity: Option<Entity>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailsResult {
    pub errors: Vec<DetailsError>,
    #[serde(rename = "type")]
    pub result_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub position: Position,
    #[serde(rename = "type")]
    pub entity_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailsError {
    pub cached: bool,
    #[serde(deserialize_with = "string_or_struct")]
    pub position: OptionalPosition,
    pub tag: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionalPosition {
    Some(Position),
    None,
}

impl FromStr for OptionalPosition {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "<no position>" || s == "<undefined>" {
            Ok(OptionalPosition::None)
        } else {
            Err(eyre::eyre!(
                "expected \"<no position>\" or \"<undefined>\" found {:?}",
                s
            ))
        }
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = eyre::Error>,
    D: de::Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> de::Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = eyre::Error>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or map or list")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            FromStr::from_str(value).map_err(de::Error::custom)
        }

        fn visit_seq<M>(self, seq: M) -> Result<T, M::Error>
        where
            M: de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub end: String,
    pub file: String,
    pub start: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramOutlineMember {
    pub name: String,
    pub position: Position,
    #[serde(rename = "type")]
    pub member_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramDefinition {
    pub location: Position,
    pub name: String,
    #[serde(rename = "scopeEnd")]
    pub scope_end: String,
    #[serde(rename = "scopeStart")]
    pub scope_start: String,
    #[serde(rename = "type")]
    pub verification_status_type: Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    pub name: String,
    #[serde(rename = "viperType")]
    pub viper_type: Option<ViperType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViperType {
    pub kind: String,
    // String or Object such as Seq[Int]
    pub typename: serde_json::Value,
}
