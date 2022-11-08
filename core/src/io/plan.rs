use crate::io::Error;
use crate::model::Plan;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    PatientId,
    PatientLastName,
    PatientFirstName,
    PatientMiddleInitial,
    PlanId,
    PlanDate,
    PlanTime,
    CourseId,
    Diagnosis,
    MdLastName,
    MdFirstName,
    MdMiddleInitial,
    MdApproveLastName,
    MdApproveFirstName,
    MdApproveMiddleInitial,
    PhyApproveLastName,
    PhyApproveFirstName,
    PhyApproveMiddleInitial,
    AuthorLastName,
    AuthorFirstName,
    AuthorMiddleInitial,
    RtpMfg,
    RtpModel,
    RtpVersion,
    RtpIfProtocol,
    RtpIfVersion,
    Crc,
}

/// Parse the data elements from a Plan Definition Record from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: plan definition record parameters
///
/// returns: Result<Plan, Error>
pub fn parse(params: &Vec<String>) -> Result<Plan, Error> {
    let mut plan = Plan::default();
    let expected_length = 28;
    let expected_keyword = "PLAN_DEF";
    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    plan.patient_id = params[Id::PatientId as usize].to_string();
    plan.patient_last_name = params[Id::PatientLastName as usize].to_string();
    plan.patient_first_name = params[Id::PatientFirstName as usize].to_string();
    plan.patient_middle_initial = params[Id::PatientMiddleInitial as usize].to_string();
    plan.plan_id = params[Id::PlanId as usize].to_string();
    plan.plan_date = params[Id::PlanDate as usize].to_string();
    plan.plan_time = params[Id::PlanTime as usize].to_string();
    plan.course_id = params[Id::CourseId as usize].to_string();
    plan.diagnosis = params[Id::Diagnosis as usize].to_string();
    plan.md_last_name = params[Id::MdLastName as usize].to_string();
    plan.md_first_name = params[Id::MdFirstName as usize].to_string();
    plan.md_middle_initial = params[Id::MdMiddleInitial as usize].to_string();
    plan.md_approve_last_name = params[Id::MdApproveLastName as usize].to_string();
    plan.md_approve_first_name = params[Id::MdApproveFirstName as usize].to_string();
    plan.md_approve_middle_initial = params[Id::MdApproveMiddleInitial as usize].to_string();
    plan.phy_approve_last_name = params[Id::PhyApproveLastName as usize].to_string();
    plan.phy_approve_first_name = params[Id::PhyApproveFirstName as usize].to_string();
    plan.phy_approve_middle_initial = params[Id::PhyApproveMiddleInitial as usize].to_string();
    plan.author_last_name = params[Id::AuthorLastName as usize].to_string();
    plan.author_first_name = params[Id::AuthorFirstName as usize].to_string();
    plan.author_middle_initial = params[Id::AuthorMiddleInitial as usize].to_string();
    plan.rtp_mfg = params[Id::RtpMfg as usize].to_string();
    plan.rtp_model = params[Id::RtpModel as usize].to_string();
    plan.rtp_version = params[Id::RtpVersion as usize].to_string();
    plan.rtp_if_protocol = params[Id::RtpIfProtocol as usize].to_string();
    plan.rtp_if_version = params[Id::RtpIfVersion as usize].to_string();
    plan.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(plan)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let params = vec![
            "PLAN_DEF".to_string(),
            "PatientId".to_string(),
            "PatientLastName".to_string(),
            "PatientFirstName".to_string(),
            "PatientMiddleInitial".to_string(),
            "PlanId".to_string(),
            "PlanDate".to_string(),
            "PlanTime".to_string(),
            "CourseId".to_string(),
            "Diagnosis".to_string(),
            "MdLastName".to_string(),
            "MdFirstName".to_string(),
            "MdMiddleInitial".to_string(),
            "MdApproveLastName".to_string(),
            "MdApproveFirstName".to_string(),
            "MdApproveMiddleInitial".to_string(),
            "PhyApproveLastName".to_string(),
            "PhyApproveFirstName".to_string(),
            "PhyApproveMiddleInitial".to_string(),
            "AuthorLastName".to_string(),
            "AuthorFirstName".to_string(),
            "AuthorMiddleInitial".to_string(),
            "RtpMfg".to_string(),
            "RtpModel".to_string(),
            "RtpVersion".to_string(),
            "RtpIfProtocol".to_string(),
            "RtpIfVersion".to_string(),
            "1234".to_string(),
        ];
        let res = super::parse(&params);
        if res.is_err() {
            eprintln!("{:#?}", res.as_ref().err());
        }
        assert!(res.is_ok());
        let plan = res.unwrap();
        assert_eq!(&plan.patient_id, "PatientId");
        assert_eq!(&plan.patient_last_name, "PatientLastName");
        assert_eq!(&plan.patient_first_name, "PatientFirstName");
        assert_eq!(&plan.patient_middle_initial, "PatientMiddleInitial");
        assert_eq!(&plan.plan_id, "PlanId");
        assert_eq!(&plan.plan_date, "PlanDate");
        assert_eq!(&plan.plan_time, "PlanTime");
        assert_eq!(&plan.course_id, "CourseId");
        assert_eq!(&plan.diagnosis, "Diagnosis");
        assert_eq!(&plan.md_last_name, "MdLastName");
        assert_eq!(&plan.md_first_name, "MdFirstName");
        assert_eq!(&plan.md_middle_initial, "MdMiddleInitial");
        assert_eq!(&plan.md_approve_last_name, "MdApproveLastName");
        assert_eq!(&plan.md_approve_first_name, "MdApproveFirstName");
        assert_eq!(&plan.md_approve_middle_initial, "MdApproveMiddleInitial");
        assert_eq!(&plan.author_last_name, "AuthorLastName");
        assert_eq!(&plan.author_first_name, "AuthorFirstName");
        assert_eq!(&plan.author_middle_initial, "AuthorMiddleInitial");
        assert_eq!(&plan.rtp_mfg, "RtpMfg");
        assert_eq!(&plan.rtp_model, "RtpModel");
        assert_eq!(&plan.rtp_version, "RtpVersion");
        assert_eq!(&plan.rtp_if_protocol, "RtpIfProtocol");
        assert_eq!(&plan.rtp_if_version, "RtpIfVersion");
        assert_eq!(plan.crc, 1234);
    }
}
