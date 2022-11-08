
#[derive(Debug, Clone)]
pub struct Plan {
    pub patient_id: String,
    pub patient_last_name: String,
    pub patient_first_name: String,
    pub patient_middle_initial: String,
    pub plan_id: String,
    pub plan_date: String,
    pub plan_time: String,
    pub course_id: String,
    pub diagnosis: String,
    pub md_last_name: String,
    pub md_first_name: String,
    pub md_middle_initial: String,
    pub md_approve_last_name: String,
    pub md_approve_first_name: String,
    pub md_approve_middle_initial: String,
    pub phy_approve_last_name: String,
    pub phy_approve_first_name: String,
    pub phy_approve_middle_initial: String,
    pub author_last_name: String,
    pub author_first_name: String,
    pub author_middle_initial: String,
    pub rtp_mfg: String,
    pub rtp_model: String,
    pub rtp_version: String,
    pub rtp_if_protocol: String,
    pub rtp_if_version: String,
    pub crc: i32,
}

impl std::default::Default for Plan {
    fn default() -> Self {
        Self {
            patient_id: "".to_string(),
            patient_last_name: "".to_string(),
            patient_first_name: "".to_string(),
            patient_middle_initial: "".to_string(),
            plan_id: "".to_string(),
            plan_date: "".to_string(),
            plan_time: "".to_string(),
            course_id: "".to_string(),
            diagnosis: "".to_string(),
            md_last_name: "".to_string(),
            md_first_name: "".to_string(),
            md_middle_initial: "".to_string(),
            md_approve_last_name: "".to_string(),
            md_approve_first_name: "".to_string(),
            md_approve_middle_initial: "".to_string(),
            phy_approve_last_name: "".to_string(),
            phy_approve_first_name: "".to_string(),
            phy_approve_middle_initial: "".to_string(),
            author_last_name: "".to_string(),
            author_first_name: "".to_string(),
            author_middle_initial: "".to_string(),
            rtp_mfg: "".to_string(),
            rtp_model: "".to_string(),
            rtp_version: "".to_string(),
            rtp_if_protocol: "".to_string(),
            rtp_if_version: "".to_string(),
            crc: 0
        }
    }
}

impl Plan {
    pub fn new() -> Plan {
        Self::default()
    }
}