use std::collections::HashMap;
use std::fmt::{self, Display};
use std::str::FromStr;

use thiserror::Error;

/// Constant for the beginning of the security.txt file.
pub const SECURITY_TXT_BEGIN: &str = "=======BEGIN SECURITY.TXT V1=======\0";
/// Constant for the end of the security.txt file.
pub const SECURITY_TXT_END: &str = "=======END SECURITY.TXT V1=======\0";

pub enum Contact {
    Email(String),
    Discord(String),
    Telegram(String),
    Twitter(String),
    Link(String),
    Other(String),
}

impl Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Contact::Discord(s) => write!(f, "Discord: {}", s),
            Contact::Email(s) => write!(f, "Email: {}", s),
            Contact::Telegram(s) => write!(f, "Telegram: {}", s),
            Contact::Twitter(s) => write!(f, "Twitter: {}", s),
            Contact::Link(s) => write!(f, "Link: {}", s),
            Contact::Other(s) => write!(f, "Other: {}", s),
        }
    }
}

pub struct SecurityTxt {
    pub name: String,
    pub project_url: String,
    pub contacts: Vec<Contact>,
    pub policy: String,
    pub preferred_languages: Vec<String>,
    pub source_code: Option<String>,
    pub source_release: Option<String>,
    pub source_revision: Option<String>,
    pub encryption: Option<String>,
    pub auditors: Vec<String>,
    pub acknowledgements: Option<String>,
    pub expiry: Option<String>,
}

impl Display for SecurityTxt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Project URL: {}", self.project_url)?;

        if !self.contacts.is_empty() {
            writeln!(f, "\nContacts:")?;
            for contact in &self.contacts {
                writeln!(f, "  {}", contact)?;
            }
        }

        writeln!(f, "\nPolicy:")?;
        writeln!(f, "{}", self.policy)?;

        if !self.preferred_languages.is_empty() {
            writeln!(f, "\nPreferred Languages:")?;
            for languages in &self.preferred_languages {
                writeln!(f, "  {}", languages)?;
            }
        }

        if let Some(source_code) = &self.source_code {
            writeln!(f, "Source code: {}", source_code)?;
        }

        if let Some(source_release) = &self.source_release {
            writeln!(f, "Source release: {}", source_release)?;
        }

        if let Some(source_revision) = &self.source_revision {
            writeln!(f, "Source revision: {}", source_revision)?;
        }

        if let Some(encryption) = &self.encryption {
            writeln!(f, "\nEncryption:")?;
            writeln!(f, "{}", encryption)?;
        }

        if !self.auditors.is_empty() {
            writeln!(f, "\nAuditors:")?;
            for auditor in &self.auditors {
                writeln!(f, "  {}", auditor)?;
            }
        }

        if let Some(acknowledegments) = &self.acknowledgements {
            writeln!(f, "\nAcknowledgements:")?;
            writeln!(f, "{}", acknowledegments)?;
        }

        if let Some(expiry) = &self.expiry {
            writeln!(f, "Expires at: {}", expiry)?;
        }
        Ok(())
    }
}

impl FromStr for Contact {
    type Err = SecurityTxtError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (typ, value) =
            s.split_once(":").ok_or_else(|| SecurityTxtError::InvalidContact(s.to_string()))?;
        let (contact_type, contact_info) = (typ.trim(), value.trim());
        match contact_type.to_ascii_lowercase().as_str() {
            "email" => Ok(Contact::Email(contact_info.to_string())),
            "discord" => Ok(Contact::Discord(contact_info.to_string())),
            "telegram" => Ok(Contact::Telegram(contact_info.to_string())),
            "twitter" => Ok(Contact::Twitter(contact_info.to_string())),
            "link" => Ok(Contact::Link(contact_info.to_string())),
            "other" => Ok(Contact::Other(contact_info.to_string())),
            _ => Err(SecurityTxtError::InvalidContact(s.to_string())),
        }
    }
}

#[derive(Debug, Error)]
pub enum SecurityTxtError {
    #[error("security.txt doesn't start with the right string")]
    InvalidSecurityTxtBegin,
    #[error("couldn't find end string")]
    EndNotFound,
    #[error("couldn't find start string")]
    StartNotFound,
    #[error("invalid field: `{0:?}`")]
    InvalidField(Vec<u8>),
    #[error("unknown field: `{0}`")]
    UnknownField(String),
    #[error("invalid value `{0:?}` for field `{1}`")]
    InvalidValue(Vec<u8>, String),
    #[error("invalid contact `{0}`")]
    InvalidContact(String),
    #[error("missing required field: `{0}`")]
    MissingField(&'static str),
    #[error("duplicate field: `{0}`")]
    DuplicateField(String),
    #[error("uneven amount of parts")]
    Uneven,
}

/// Parses a security.txt from the start of `data`.
pub fn parse(data: &[u8]) -> Result<SecurityTxt, SecurityTxtError> {
    let Some(data) = data.strip_prefix(SECURITY_TXT_BEGIN.as_bytes()) else {
        return Err(SecurityTxtError::InvalidSecurityTxtBegin);
    };

    let data = match memchr::memmem::find(data, SECURITY_TXT_END.as_bytes()) {
        Some(end) => &data[..end],
        None => return Err(SecurityTxtError::EndNotFound),
    };

    let mut attributes = HashMap::<String, String>::new();
    let mut field: Option<String> = None;
    for part in data.split(|&b| b == 0) {
        if let Some(ref f) = field {
            let value = std::str::from_utf8(part)
                .map_err(|_| SecurityTxtError::InvalidValue(part.to_vec(), f.clone()))?;
            attributes.insert(f.clone(), value.to_string());
            field = None;
        } else {
            field = Some({
                let field = std::str::from_utf8(part)
                    .map_err(|_| SecurityTxtError::InvalidField(part.to_vec()))?
                    .to_string();
                if attributes.contains_key(&field) {
                    return Err(SecurityTxtError::DuplicateField(field));
                }
                field
            });
        }
    }

    let name = attributes.remove("name").ok_or(SecurityTxtError::MissingField("name"))?;
    let project_url =
        attributes.remove("project_url").ok_or(SecurityTxtError::MissingField("project_url"))?;
    let source_code = attributes.remove("source_code");
    let source_release = attributes.remove("source_release");
    let source_revision = attributes.remove("source_revision");
    let expiry = attributes.remove("expiry");
    let preferred_languages = attributes
        .remove("preferred_languages")
        .unwrap_or_default()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    let contacts: Result<Vec<_>, SecurityTxtError> = attributes
        .remove("contacts")
        .ok_or(SecurityTxtError::MissingField("contacts"))?
        .split(",")
        .map(|s| Contact::from_str(s.trim()))
        .collect();
    let contacts = contacts?;
    let auditors: Vec<_> = attributes
        .remove("auditors")
        .unwrap_or_default()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    let encryption = attributes.remove("encryption");
    let acknowledgements = attributes.remove("acknowledgements");
    let policy = attributes.remove("policy").ok_or(SecurityTxtError::MissingField("policy"))?;

    if !attributes.is_empty() {
        return Err(SecurityTxtError::UnknownField(attributes.keys().next().unwrap().clone()));
    }

    Ok(SecurityTxt {
        name,
        project_url,
        source_code,
        source_release,
        source_revision,
        expiry,
        preferred_languages,
        contacts,
        auditors,
        encryption,
        acknowledgements,
        policy,
    })
}

/// Finds and parses the security.txt in the haystack.
pub fn find_and_parse(data: &[u8]) -> Result<SecurityTxt, SecurityTxtError> {
    let start = match memchr::memmem::find(data, SECURITY_TXT_BEGIN.as_bytes()) {
        Some(i) => i,
        None => return Err(SecurityTxtError::StartNotFound),
    };
    parse(&data[start..])
}
