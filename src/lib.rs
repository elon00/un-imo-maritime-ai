use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UNAgencyState {
    pub agency_id: u32,
    pub agency_name: String,
    pub project_name: String,
    pub portfolio: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultilateralAuditRecord {
    pub audit_id: String,
    pub timestamp: u64,
    pub merkle_root_hash: String,
    pub verified_by_pqc: bool,
}

pub fn get_metadata() -> UNAgencyState {
    UNAgencyState {
        agency_id: 8,
        agency_name: "International Maritime Organization (IMO)".to_string(),
        project_name: "IMO MaritimeTrace AI".to_string(),
        portfolio: "International Maritime Navigation & Oceans".to_string(),
        strategy: "Tokenless Cryptographic Audit Anchor -- Flag State Vessel Registration and Carbon Intensity Telemetry".to_string(),
        target_chain: "Internet Computer (ICP) + Algorand".to_string(),
        is_active: true,
    }
}
