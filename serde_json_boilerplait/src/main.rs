use serde::de::{Deserialize, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct};

struct Token {
    id: i32,
    name: String,
    process: bool,
}

impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Token", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("process", &self.process)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for Token {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TokenVisitor;

        impl<'de> Visitor<'de> for TokenVisitor {
            type Value = Token;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Token")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut process = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "id" => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        "name" => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        "process" => {
                            if process.is_some() {
                                return Err(serde::de::Error::duplicate_field("process"));
                            }
                            process = Some(map.next_value()?);
                        }
                        _ => {
                            // Ignore unknown fields
                            let _ = map.next_value::<serde::de::IgnoredAny>();
                        }
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let process = process.ok_or_else(|| serde::de::Error::missing_field("process"))?;

                Ok(Token { id, name, process })
            }
        }
        deserializer.deserialize_struct("Token", &["id", "name", "process"], TokenVisitor)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token: id {}, name: {}, process: {}",
            self.id, self.name, self.process
        )
    }
}

fn main() {
    let candidate = Token {
        id: 1,
        name: "Marija".to_owned(),
        process: true,
    };

    let serialize = serde_json::to_string(&candidate).unwrap();
    println!("{}", serialize);

    let deserialize: Token = serde_json::from_str(&serialize).unwrap();
    println!("{}", deserialize);
}
