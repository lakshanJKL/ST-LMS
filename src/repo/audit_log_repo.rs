use chrono::Utc;
use mongodb::{Collection, Database};
use crate::exceptions::errors::SystemError;
use crate::models::audit_log_model::AuditLog;

pub struct AuditLogRepo {
    collection: Collection<AuditLog>,
}

impl AuditLogRepo {
    pub async fn new(db: &Database) -> Self {
        let collection = db.collection("audit_logs");
        Self { collection }
    }

    // add new log
    pub async fn log_action(&self, user: Option<String>, actions: String, detail: String) -> Result<(), SystemError> {
        let new_log = AuditLog {
            user_id: user,
            action: actions,
            timestamp: Utc::now().timestamp(),
            details: detail,
        };

        self.collection.insert_one(new_log).await?;
        Ok(())
    }
}