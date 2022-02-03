use crate::config;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;
use uuid::Uuid;

/// https://raw.githubusercontent.com/ga4gh-discovery/ga4gh-service-info/v1.0.0/service-info.yaml#/paths/~1service-info
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub r#type: ServiceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub organization: Organization,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    pub version: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ServiceType {
    pub group: String,
    pub artifact: String,
    pub version: String,
}

impl Default for ServiceType {
    fn default() -> Self {
        Self {
            group: "gh-trs".to_string(),
            artifact: "gh-trs".to_string(),
            version: "2.0.1".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Organization {
    pub name: String,
    pub url: Url,
}

impl ServiceInfo {
    fn new(config: &config::Config, owner: impl AsRef<str>, name: impl AsRef<str>) -> Result<Self> {
        let created_at = Utc::now();
        Ok(Self {
            id: format!("{}.{}.gh-trs", name.as_ref(), owner.as_ref()),
            name: format!("gh-trs {}/{}", owner.as_ref(), name.as_ref()),
            r#type: ServiceType::default(),
            description: Some("The GA4GH TRS API generated by gh-trs".to_string()),
            organization: Organization {
                name: config.authors[0].github_account.clone(),
                url: Url::parse(&format!(
                    "https://github.com/{}",
                    config.authors[0].github_account
                ))?,
            },
            contact_url: None::<Url>,
            documentation_url: None::<Url>,
            created_at: Some(created_at),
            updated_at: Some(created_at.clone()),
            environment: None::<String>,
            version: created_at.format("%Y%m%d%H%M%S").to_string(),
        })
    }

    pub fn new_or_update(
        prev: Option<Self>,
        config: &config::Config,
        owner: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Result<Self> {
        let mut service_info = Self::new(config, owner, name)?;
        if let Some(prev) = prev {
            service_info.id = prev.id.clone();
            service_info.name = prev.name.clone();
            service_info.r#type = prev.r#type.clone();
            service_info.description = prev.description.clone();
            service_info.organization = prev.organization.clone();
            service_info.contact_url = prev.contact_url.clone();
            service_info.documentation_url = prev.documentation_url.clone();
            service_info.created_at = prev.created_at.clone();
            service_info.environment = prev.environment.clone();
        }
        Ok(service_info)
    }
}

// --- GA4GH TRS API v2.0.1 type definition ---
// https://editor.swagger.io/?url=https://raw.githubusercontent.com/ga4gh/tool-registry-schemas/develop/openapi/openapi.yaml

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Checksum {
    pub checksum: String,
    pub r#type: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum FileType {
    TestFile,
    PrimaryDescriptor,
    SecondaryDescriptor,
    Containerfile,
    Other,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ToolFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<FileType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ToolClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Tool {
    pub url: Url,
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    pub organization: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub tool_class: ToolClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_checker: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checker_url: Option<Url>,
    pub versions: Vec<ToolVersion>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ToolVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub url: Url,
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_production: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor_type: Option<Vec<DescriptorType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containerfile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_source: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_apps: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ImageData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<Checksum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<ImageType>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ImageType {
    Docker,
    Singularity,
    Conda,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum DescriptorType {
    Cwl,
    Wdl,
    Nfl,
    Smk, // extended
    Galaxy,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum DescriptorTypeWithPlain {
    Cwl,
    Wdl,
    Nfl,
    Smk, // extended
    Galaxy,
    PlainCwl,
    PlainWdl,
    PlainNfl,
    PlainSmk, // extended
    PlainGalaxy,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FileWrapper {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<Vec<Checksum>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}

// --- Type definition end ---

impl Default for ToolClass {
    fn default() -> Self {
        ToolClass {
            id: Some("workflow".to_string()),
            name: Some("Workflow".to_string()),
            description: Some("A computational workflow".to_string()),
        }
    }
}

impl ToolVersion {
    fn new(config: &config::Config, owner: impl AsRef<str>, name: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            author: Some(
                config
                    .authors
                    .iter()
                    .map(|a| a.github_account.clone())
                    .collect::<Vec<String>>(),
            ),
            name: Some(config.workflow.name.clone()),
            url: Url::parse(&format!(
                "https://{}.github.io/{}/tools/{}/versions/{}",
                owner.as_ref(),
                name.as_ref(),
                config.id.to_string(),
                &config.version
            ))?,
            id: config.id,
            is_production: None::<bool>,
            images: None::<Vec<ImageData>>,
            descriptor_type: Some(vec![DescriptorType::new(
                &config
                    .workflow
                    .language
                    .r#type
                    .clone()
                    .ok_or(anyhow!("No language type"))?,
            )]),
            containerfile: None::<bool>,
            meta_version: None::<String>,
            verified: None::<bool>,
            verified_source: None::<Vec<String>>,
            signed: None::<bool>,
            included_apps: None::<Vec<String>>,
        })
    }
}

impl DescriptorType {
    fn new(wf_type: &config::LanguageType) -> Self {
        match wf_type {
            config::LanguageType::Cwl => DescriptorType::Cwl,
            config::LanguageType::Wdl => DescriptorType::Wdl,
            config::LanguageType::Nfl => DescriptorType::Nfl,
            config::LanguageType::Smk => DescriptorType::Smk,
        }
    }
}
