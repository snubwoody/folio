use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename = "Package")]
#[serde(rename_all = "PascalCase")]
pub struct AppxManifest {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "@xmlns:uap")]
    pub xmlns_uap: String,
    #[serde(rename = "@xmlns:rescap")]
    pub xmlns_rescap: String,
    #[serde(rename = "@IgnorableNamespaces")]
    pub ignorable_namespaces: String,
    pub identity: Identity,
    pub properties: Properties,
    pub dependencies: Dependencies,
    pub resources: Resources,
    /// Although each package can contain one or more apps,
    /// packages that contain multiple apps won't pass the Store
    /// certification process.
    pub applications: Applications,
    pub capabilities: Capabilities,
}

impl AppxManifest {
    /// Creates a new appxmanifest.
    pub fn new() -> AppxManifest {
        Self {
            xmlns: String::from("http://schemas.microsoft.com/appx/manifest/foundation/windows10"),
            xmlns_uap: String::from("http://schemas.microsoft.com/appx/manifest/uap/windows10"),
            xmlns_rescap: String::from(
                "http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities",
            ),
            ignorable_namespaces: String::from("uap rescap"),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Identity {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Version")]
    pub version: String,
    #[serde(rename = "@Publisher")]
    pub publisher: String,
    #[serde(rename = "@ProcessorArchitecture")]
    pub processor_architecture: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Applications {
    #[serde(rename = "Application")]
    pub applications: Vec<Application>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Application {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "@Executable")]
    pub executable: String,
    #[serde(rename = "@EntryPoint")]
    pub entry_point: String,
    #[serde(rename = "uap:VisualElements")]
    pub visual_elements: VisualElements,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VisualElements {
    #[serde(rename = "@DisplayName")]
    pub display_name: String,
    #[serde(rename = "@Description")]
    pub description: String,
    #[serde(rename = "@BackgroundColor")]
    pub background_color: String,
    #[serde(rename = "@Square150x150Logo")]
    pub square_150_logo: String,
    #[serde(rename = "@Square44x44Logo")]
    pub square_44_logo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resources {
    #[serde(rename = "Resource")]
    pub resources: Vec<Resource>,
}

impl Default for Resources {
    fn default() -> Self {
        let resource = Resource {
            language: "en-GB".to_string(),
        };

        Self {
            resources: vec![resource],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Resource {
    #[serde(rename = "@Language")]
    language: String,
}

// TODO: test casing
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Properties {
    pub display_name: String,
    pub publisher_display_name: String,
    pub logo: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct Dependencies {
    pub target_device_family: TargetDeviceFamily,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Capabilities {
    #[serde(rename = "rescap:Capability")]
    capabilities: Vec<Capability>,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            capabilities: vec![Capability::default()],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Capability {
    #[serde(rename = "@Name")]
    name: String,
}

impl Default for Capability {
    fn default() -> Self {
        Self {
            name: String::from("runFullTrust"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct TargetDeviceFamily {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@MinVersion")]
    min_version: String,
    #[serde(rename = "@MaxVersionTested")]
    max_version: String,
}

impl Default for TargetDeviceFamily {
    fn default() -> Self {
        // (i think) this is the minimum version supported by msix.
        let min_version = "10.0.17763.0";
        // just an arbitrary version that is recent enough.
        let max_version = "10.0.22621.0";

        Self {
            name: String::from("Windows.Desktop"),
            min_version: String::from(min_version),
            max_version: String::from(max_version),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn manifest_namespace() {
        let manifest = AppxManifest::new();
        assert_eq!(
            manifest.xmlns,
            "http://schemas.microsoft.com/appx/manifest/foundation/windows10"
        );
        assert_eq!(
            manifest.xmlns_uap,
            "http://schemas.microsoft.com/appx/manifest/uap/windows10"
        );
        assert_eq!(
            manifest.xmlns_rescap,
            "http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities"
        );
        assert_eq!(manifest.ignorable_namespaces, "uap rescap");
    }
}
