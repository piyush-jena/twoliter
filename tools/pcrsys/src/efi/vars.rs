//! EFI variable types and parsing.

use super::esl::{EFI_GLOBAL_VARIABLE_GUID, EFI_IMAGE_SECURITY_DATABASE_GUID};
use crate::error::Result;
use serde::Deserialize;
use snafu::{ensure_whatever, OptionExt, ResultExt, Whatever};
use uuid::Uuid;

/// The Secure Boot variables firmware measures into PCR 7, in that order, with their GUIDs.
pub const SECURE_BOOT_VARIABLES: [(&str, Uuid); 4] = [
    ("PK", EFI_GLOBAL_VARIABLE_GUID),
    ("KEK", EFI_GLOBAL_VARIABLE_GUID),
    ("db", EFI_IMAGE_SECURITY_DATABASE_GUID),
    ("dbx", EFI_IMAGE_SECURITY_DATABASE_GUID),
];

/// Raw EFI variables before validation.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RawEfiVars {
    /// Format version (ignored, present in some efi-vars.json files).
    #[allow(dead_code)]
    version: Option<u32>,
    /// List of EFI variables.
    variables: Vec<EfiVar>,
}

/// Collection of EFI variables from efi-vars.json.
/// Validated on parse to ensure required variables (PK, KEK, db, dbx) are present.
#[derive(Debug, Deserialize)]
#[serde(try_from = "RawEfiVars")]
pub struct EfiVars {
    /// List of EFI variables.
    pub variables: Vec<EfiVar>,
}

impl TryFrom<RawEfiVars> for EfiVars {
    type Error = Whatever;

    fn try_from(raw: RawEfiVars) -> std::result::Result<Self, Self::Error> {
        for (name, guid) in SECURE_BOOT_VARIABLES {
            let var = raw
                .variables
                .iter()
                .find(|v| v.name == name)
                .whatever_context(format!("missing required variable: {name}"))?;
            let guid = guid.to_string();
            ensure_whatever!(
                var.guid == guid,
                "{} has GUID {} but expected {}",
                name,
                var.guid,
                guid
            );
        }
        Ok(EfiVars {
            variables: raw.variables,
        })
    }
}

/// A single EFI variable with name, GUID, and hex-encoded data.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EfiVar {
    /// Variable name (e.g., "PK", "KEK", "db", "dbx").
    pub name: String,
    /// Variable GUID as string.
    pub guid: String,
    /// EFI variable attributes (ignored).
    #[serde(default)]
    #[allow(dead_code)]
    attr: Option<u32>,
    /// Hex-encoded variable data.
    pub data: String,
    /// Hex-encoded EFI_TIME timestamp (ignored).
    #[serde(default)]
    #[allow(dead_code)]
    time: Option<String>,
}

#[cfg(test)]
impl EfiVar {
    /// Create a new EFI variable with the given name, GUID, and hex-encoded data.
    pub fn new(name: impl Into<String>, guid: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            guid: guid.into(),
            attr: None,
            data: data.into(),
            time: None,
        }
    }
}

impl EfiVars {
    /// Find a variable by name.
    pub fn get(&self, name: &str) -> Option<&EfiVar> {
        self.variables.iter().find(|v| v.name == name)
    }
}

/// The measured Secure Boot variables' data, decoded once, in `SECURE_BOOT_VARIABLES` order.
#[derive(Debug, Default)]
pub struct SecureBootVars([Vec<u8>; 4]);

impl SecureBootVars {
    /// Decode the measured variables from efi-vars.json.
    pub fn from_json(vars: &EfiVars) -> Result<Self> {
        Self::collect(|name| {
            let var = vars
                .get(name)
                .whatever_context(format!("{name} variable not found"))?;
            hex::decode(&var.data).whatever_context(format!("invalid {name} hex"))
        })
    }

    /// Take the measured variables from a variable store's `(name, data)` pairs.
    /// The first variable of a name wins.
    pub fn from_store(mut variables: Vec<(String, Vec<u8>)>) -> Result<Self> {
        Self::collect(|name| {
            let index = variables
                .iter()
                .position(|(variable, _)| variable == name)
                .whatever_context(format!("{name} variable not found"))?;
            Ok(variables.remove(index).1)
        })
    }

    /// Each measured variable's name, GUID, and data, in measurement order.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, Uuid, &[u8])> {
        SECURE_BOOT_VARIABLES
            .iter()
            .zip(&self.0)
            .map(|(&(name, guid), data)| (name, guid, data.as_slice()))
    }

    fn collect(mut data: impl FnMut(&str) -> Result<Vec<u8>>) -> Result<Self> {
        let [pk, kek, db, dbx] = SECURE_BOOT_VARIABLES.map(|(name, _)| name);
        Ok(Self([data(pk)?, data(kek)?, data(db)?, data(dbx)?]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_efi_vars_get() {
        let global = EFI_GLOBAL_VARIABLE_GUID.to_string();
        let vars = EfiVars {
            variables: vec![
                EfiVar::new("PK", &global, "00"),
                EfiVar::new("KEK", &global, "01"),
            ],
        };
        assert!(vars.get("PK").is_some());
        assert!(vars.get("KEK").is_some());
        assert!(vars.get("db").is_none());
    }

    #[test]
    fn test_validate_ok() {
        let global = EFI_GLOBAL_VARIABLE_GUID.to_string();
        let secdb = EFI_IMAGE_SECURITY_DATABASE_GUID.to_string();
        let raw = RawEfiVars {
            version: None,
            variables: vec![
                EfiVar::new("PK", &global, "00"),
                EfiVar::new("KEK", &global, "01"),
                EfiVar::new("db", &secdb, "02"),
                EfiVar::new("dbx", &secdb, "03"),
            ],
        };
        assert!(EfiVars::try_from(raw).is_ok());
    }

    /// Helper to build RawEfiVars with one variable having a wrong GUID.
    fn vars_with_wrong_guid(name: &str) -> RawEfiVars {
        let wrong = "wrong-guid";
        let global = EFI_GLOBAL_VARIABLE_GUID.to_string();
        let secdb = EFI_IMAGE_SECURITY_DATABASE_GUID.to_string();
        RawEfiVars {
            version: None,
            variables: vec![
                EfiVar::new("PK", if name == "PK" { wrong } else { &global }, "00"),
                EfiVar::new("KEK", if name == "KEK" { wrong } else { &global }, "01"),
                EfiVar::new("db", if name == "db" { wrong } else { &secdb }, "02"),
                EfiVar::new("dbx", if name == "dbx" { wrong } else { &secdb }, "03"),
            ],
        }
    }

    #[test_case("PK" ; "wrong PK guid")]
    #[test_case("KEK" ; "wrong KEK guid")]
    #[test_case("db" ; "wrong db guid")]
    #[test_case("dbx" ; "wrong dbx guid")]
    fn test_validate_wrong_guid(name: &str) {
        let raw = vars_with_wrong_guid(name);
        let err = EfiVars::try_from(raw).unwrap_err().to_string();
        assert!(err.contains(name) && err.contains("GUID"));
    }

    /// Helper to build RawEfiVars missing one variable.
    fn vars_missing(name: &str) -> RawEfiVars {
        let global = EFI_GLOBAL_VARIABLE_GUID.to_string();
        let secdb = EFI_IMAGE_SECURITY_DATABASE_GUID.to_string();
        let mut vars = vec![
            EfiVar::new("PK", &global, "00"),
            EfiVar::new("KEK", &global, "01"),
            EfiVar::new("db", &secdb, "02"),
            EfiVar::new("dbx", &secdb, "03"),
        ];
        vars.retain(|v| v.name != name);
        RawEfiVars {
            version: None,
            variables: vars,
        }
    }

    #[test_case("PK" ; "missing PK")]
    #[test_case("KEK" ; "missing KEK")]
    #[test_case("db" ; "missing db")]
    #[test_case("dbx" ; "missing dbx")]
    fn test_validate_missing(name: &str) {
        let raw = vars_missing(name);
        let err = EfiVars::try_from(raw).unwrap_err().to_string();
        assert!(err.contains("missing") && err.contains(name));
    }

    #[test]
    fn secure_boot_vars_decode_each_measured_variable_once() {
        let global = EFI_GLOBAL_VARIABLE_GUID.to_string();
        let secdb = EFI_IMAGE_SECURITY_DATABASE_GUID.to_string();
        let vars = EfiVars {
            variables: vec![
                EfiVar::new("dbx", &secdb, "03"),
                EfiVar::new("PK", &global, "00"),
                EfiVar::new("Other", &global, "zz"),
                EfiVar::new("KEK", &global, "01"),
                EfiVar::new("db", &secdb, "02"),
            ],
        };
        let vars = SecureBootVars::from_json(&vars).unwrap();
        let measured: Vec<_> = vars
            .iter()
            .map(|(name, _, data)| (name, data.to_vec()))
            .collect();
        assert_eq!(
            measured,
            [
                ("PK", vec![0]),
                ("KEK", vec![1]),
                ("db", vec![2]),
                ("dbx", vec![3])
            ]
        );
    }

    #[test]
    fn secure_boot_vars_reject_missing_and_invalid_variables() {
        let global = EFI_GLOBAL_VARIABLE_GUID.to_string();
        let vars = EfiVars {
            variables: vec![EfiVar::new("PK", &global, "ZZZZ")],
        };
        let err = SecureBootVars::from_json(&vars).unwrap_err();
        assert!(err.to_string().contains("invalid PK hex"), "{err}");
        let err = SecureBootVars::from_store(vec![("PK".to_string(), vec![0])]).unwrap_err();
        assert!(err.to_string().contains("KEK variable not found"), "{err}");
    }
}
