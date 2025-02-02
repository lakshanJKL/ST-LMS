use serde::{Deserialize, Serialize};

/*
 An audit log is a record of events or actions that occur in this application,
 typically used for tracking changes, monitoring user activity, and ensuring accountability.
 audit log use for applications handling sensitive data
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,    // ID of the user who performed the action
    pub action: String,             // Action performed (e.g., "create_user")
    pub timestamp: i64,             // Timestamp of the action
    pub details: String,             // Additional details (e.g., request body, changes made)
}

