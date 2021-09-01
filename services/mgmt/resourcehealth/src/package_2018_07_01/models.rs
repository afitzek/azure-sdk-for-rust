#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Events {
    pub value: Vec<Event>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<event::Properties>,
}
pub mod event {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
        pub event_type: Option<properties::EventType>,
        #[serde(rename = "eventSource", default, skip_serializing_if = "Option::is_none")]
        pub event_source: Option<properties::EventSource>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub summary: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub header: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<properties::Level>,
        #[serde(rename = "eventLevel", default, skip_serializing_if = "Option::is_none")]
        pub event_level: Option<properties::EventLevel>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub article: Option<properties::Article>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub links: Vec<Link>,
        #[serde(rename = "impactStartTime", default, skip_serializing_if = "Option::is_none")]
        pub impact_start_time: Option<String>,
        #[serde(rename = "impactMitigationTime", default, skip_serializing_if = "Option::is_none")]
        pub impact_mitigation_time: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub impact: Vec<Impact>,
        #[serde(rename = "recommendedActions", default, skip_serializing_if = "Option::is_none")]
        pub recommended_actions: Option<properties::RecommendedActions>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub faqs: Vec<Faq>,
        #[serde(rename = "isHIR", default, skip_serializing_if = "Option::is_none")]
        pub is_hir: Option<bool>,
        #[serde(rename = "enableMicrosoftSupport", default, skip_serializing_if = "Option::is_none")]
        pub enable_microsoft_support: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(rename = "platformInitiated", default, skip_serializing_if = "Option::is_none")]
        pub platform_initiated: Option<bool>,
        #[serde(rename = "enableChatWithUs", default, skip_serializing_if = "Option::is_none")]
        pub enable_chat_with_us: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<i64>,
        #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
        pub last_update_time: Option<String>,
        #[serde(rename = "hirStage", default, skip_serializing_if = "Option::is_none")]
        pub hir_stage: Option<String>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum EventType {
            ServiceIssue,
            PlannedMaintenance,
            HealthAdvisory,
            #[serde(rename = "RCA")]
            Rca,
            EmergingIssues,
            SecurityAdvisory,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum EventSource {
            ResourceHealth,
            ServiceHealth,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Active,
            Resolved,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Level {
            Critical,
            Warning,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum EventLevel {
            Critical,
            Error,
            Warning,
            Informational,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct Article {
            #[serde(rename = "articleContent", default, skip_serializing_if = "Option::is_none")]
            pub article_content: Option<String>,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct RecommendedActions {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub message: Option<String>,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub actions: Vec<serde_json::Value>,
            #[serde(rename = "localeCode", default, skip_serializing_if = "Option::is_none")]
            pub locale_code: Option<String>,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<link::Type>,
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<link::DisplayText>,
    #[serde(rename = "extensionName", default, skip_serializing_if = "Option::is_none")]
    pub extension_name: Option<String>,
    #[serde(rename = "bladeName", default, skip_serializing_if = "Option::is_none")]
    pub blade_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
pub mod link {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Button,
        Hyperlink,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DisplayText {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
        pub localized_value: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Faq {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    #[serde(rename = "localeCode", default, skip_serializing_if = "Option::is_none")]
    pub locale_code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Impact {
    #[serde(rename = "impactedService", default, skip_serializing_if = "Option::is_none")]
    pub impacted_service: Option<String>,
    #[serde(rename = "impactedRegions", default, skip_serializing_if = "Vec::is_empty")]
    pub impacted_regions: Vec<ImpactedServiceRegion>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImpactedServiceRegion {
    #[serde(rename = "impactedRegion", default, skip_serializing_if = "Option::is_none")]
    pub impacted_region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<impacted_service_region::Status>,
    #[serde(rename = "impactedSubscriptions", default, skip_serializing_if = "Vec::is_empty")]
    pub impacted_subscriptions: Vec<String>,
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub updates: Vec<Update>,
}
pub mod impacted_service_region {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Active,
        Resolved,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "updateDateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_date_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityStatusListResult {
    pub value: Vec<AvailabilityStatus>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<availability_status::Properties>,
}
pub mod availability_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "availabilityState", default, skip_serializing_if = "Option::is_none")]
        pub availability_state: Option<properties::AvailabilityState>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub summary: Option<String>,
        #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
        pub detailed_status: Option<String>,
        #[serde(rename = "reasonType", default, skip_serializing_if = "Option::is_none")]
        pub reason_type: Option<String>,
        #[serde(rename = "rootCauseAttributionTime", default, skip_serializing_if = "Option::is_none")]
        pub root_cause_attribution_time: Option<String>,
        #[serde(rename = "healthEventType", default, skip_serializing_if = "Option::is_none")]
        pub health_event_type: Option<String>,
        #[serde(rename = "healthEventCause", default, skip_serializing_if = "Option::is_none")]
        pub health_event_cause: Option<String>,
        #[serde(rename = "healthEventCategory", default, skip_serializing_if = "Option::is_none")]
        pub health_event_category: Option<String>,
        #[serde(rename = "healthEventId", default, skip_serializing_if = "Option::is_none")]
        pub health_event_id: Option<String>,
        #[serde(rename = "resolutionETA", default, skip_serializing_if = "Option::is_none")]
        pub resolution_eta: Option<String>,
        #[serde(rename = "occurredTime", default, skip_serializing_if = "Option::is_none")]
        pub occurred_time: Option<String>,
        #[serde(rename = "reasonChronicity", default, skip_serializing_if = "Option::is_none")]
        pub reason_chronicity: Option<properties::ReasonChronicity>,
        #[serde(rename = "reportedTime", default, skip_serializing_if = "Option::is_none")]
        pub reported_time: Option<String>,
        #[serde(rename = "recentlyResolved", default, skip_serializing_if = "Option::is_none")]
        pub recently_resolved: Option<properties::RecentlyResolved>,
        #[serde(rename = "recommendedActions", default, skip_serializing_if = "Vec::is_empty")]
        pub recommended_actions: Vec<RecommendedAction>,
        #[serde(rename = "serviceImpactingEvents", default, skip_serializing_if = "Vec::is_empty")]
        pub service_impacting_events: Vec<ServiceImpactingEvent>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum AvailabilityState {
            Available,
            Unavailable,
            Degraded,
            Unknown,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ReasonChronicity {
            Transient,
            Persistent,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct RecentlyResolved {
            #[serde(rename = "unavailableOccurredTime", default, skip_serializing_if = "Option::is_none")]
            pub unavailable_occurred_time: Option<String>,
            #[serde(rename = "resolvedTime", default, skip_serializing_if = "Option::is_none")]
            pub resolved_time: Option<String>,
            #[serde(rename = "unavailabilitySummary", default, skip_serializing_if = "Option::is_none")]
            pub unavailability_summary: Option<String>,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendedAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "actionUrl", default, skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[serde(rename = "actionUrlText", default, skip_serializing_if = "Option::is_none")]
    pub action_url_text: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceImpactingEvent {
    #[serde(rename = "eventStartTime", default, skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<String>,
    #[serde(rename = "eventStatusLastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub event_status_last_modified_time: Option<String>,
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<service_impacting_event::Status>,
    #[serde(rename = "incidentProperties", default, skip_serializing_if = "Option::is_none")]
    pub incident_properties: Option<service_impacting_event::IncidentProperties>,
}
pub mod service_impacting_event {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Status {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct IncidentProperties {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub service: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "incidentType", default, skip_serializing_if = "Option::is_none")]
        pub incident_type: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusBanner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<String>,
    #[serde(rename = "lastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImpactedRegion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmergingIssueImpact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<ImpactedRegion>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusActiveEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<status_active_event::Severity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<status_active_event::Stage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[serde(rename = "lastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub impacts: Vec<EmergingIssueImpact>,
}
pub mod status_active_event {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Information,
        Warning,
        Error,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Stage {
        Active,
        Resolve,
        Archived,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmergingIssuesGetResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EmergingIssue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmergingIssue {
    #[serde(rename = "refreshTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub refresh_timestamp: Option<String>,
    #[serde(rename = "statusBanners", default, skip_serializing_if = "Vec::is_empty")]
    pub status_banners: Vec<StatusBanner>,
    #[serde(rename = "statusActiveEvents", default, skip_serializing_if = "Vec::is_empty")]
    pub status_active_events: Vec<StatusActiveEvent>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmergingIssueListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EmergingIssuesGetResult>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataEntityListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MetadataEntity>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataEntity {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetadataEntityProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataEntityProperties {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(rename = "applicableScenarios", default, skip_serializing_if = "Vec::is_empty")]
    pub applicable_scenarios: Vec<String>,
    #[serde(rename = "supportedValues", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_values: Vec<MetadataSupportedValueDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSupportedValueDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
