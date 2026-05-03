use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DomainEventKind {
    ConfigXmlChanged,
    CfeChanged,
    MetadataChanged,
    FormChanged,
    ModuleChanged,
    RoleChanged,
    SkdChanged,
    MxlChanged,
    SubsystemChanged,
    TemplateChanged,
    SourceSetChanged,
    BuildCompleted,
}

impl DomainEventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ConfigXmlChanged => "ConfigXmlChanged",
            Self::CfeChanged => "CfeChanged",
            Self::MetadataChanged => "MetadataChanged",
            Self::FormChanged => "FormChanged",
            Self::ModuleChanged => "ModuleChanged",
            Self::RoleChanged => "RoleChanged",
            Self::SkdChanged => "SkdChanged",
            Self::MxlChanged => "MxlChanged",
            Self::SubsystemChanged => "SubsystemChanged",
            Self::TemplateChanged => "TemplateChanged",
            Self::SourceSetChanged => "SourceSetChanged",
            Self::BuildCompleted => "BuildCompleted",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DomainEvent {
    pub kind: DomainEventKind,
    pub artifact: String,
}

impl DomainEvent {
    pub fn new(kind: DomainEventKind, artifact: impl Into<String>) -> Self {
        Self {
            kind,
            artifact: artifact.into(),
        }
    }

    pub fn name(&self) -> &'static str {
        self.kind.as_str()
    }
}
