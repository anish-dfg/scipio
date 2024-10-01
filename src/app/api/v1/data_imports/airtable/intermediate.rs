use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::services::storage::mentors::CreateMentor;
use crate::services::storage::nonprofits::CreateNonprofit;
use crate::services::storage::types::{
    AgeRange, ClientSize, Ethnicity, Fli, Gender, ImpactCause, Lgbt, MentorExperienceLevel,
    MentorYearsExperience, StudentStage, VolunteerHearAbout,
};
use crate::services::storage::volunteers::CreateVolunteer;

#[derive(Builder, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IntermediateNonprofitData {
    #[builder(setter(into))]
    #[serde(rename = "FirstName")]
    pub representative_first_name: String,
    #[builder(setter(into))]
    #[serde(rename = "LastName")]
    pub representative_last_name: String,
    #[builder(setter(into))]
    #[serde(rename = "JobTitle")]
    pub representative_job_title: String,
    #[builder(setter(into))]
    #[serde(rename = "NonprofitEmail")]
    pub email: String,
    #[serde(rename = "Cc")]
    #[builder(setter(into), default = "None")]
    pub email_cc: Option<String>,
    #[builder(setter(into))]
    pub phone: String,
    #[builder(setter(into))]
    pub org_name: String,
    #[builder(setter(into))]
    pub project_name: String,
    #[builder(setter(into), default = "None")]
    pub org_website: Option<String>,
    #[serde(rename = "CountryHQ")]
    #[builder(setter(into), default = "None")]
    pub country_hq: Option<String>,
    #[serde(rename = "StateHQ")]
    #[builder(setter(into), default = "None")]
    pub us_state_hq: Option<String>,
    #[builder(setter(into))]
    pub address: String,
    pub size: ClientSize,
    pub impact_causes: Option<Vec<String>>,
}

impl From<IntermediateNonprofitData> for CreateNonprofit {
    fn from(value: IntermediateNonprofitData) -> Self {
        CreateNonprofit {
            representative_first_name: value.representative_first_name,
            representative_last_name: value.representative_last_name,
            representative_job_title: value.representative_job_title,
            email: value.email,
            email_cc: value.email_cc,
            phone: value.phone,
            org_name: value.org_name,
            project_name: value.project_name,
            org_website: value.org_website,
            country_hq: value.country_hq,
            us_state_hq: value.us_state_hq,
            address: value.address,
            size: value.size,
            impact_causes: value
                .impact_causes
                .unwrap_or(vec!["Other".to_owned()])
                .iter()
                .map(|c| match c.as_str() {
                    "reco1zHRYv8lTQDaI" => ImpactCause::Animals,
                    "recXhhTPsuQ2PMjU4" => ImpactCause::CareerAndProfessionalDevelopment,
                    "recvWKilRRABCcHuI" => ImpactCause::DisasterRelief,
                    "recYfRNFDpm2nedjM" => ImpactCause::Education,
                    "recOlWiJTppnQwnll" => ImpactCause::EnvironmentAndSustainability,
                    "recix0Y5qCXYfZGRz" => ImpactCause::FaithAndReligion,
                    "recKs8kboTORruStC" => ImpactCause::HealthAndMedicine,
                    "recEmtYMgeOlPeOVQ" => ImpactCause::GlobalRelations,
                    "reczSSbvdW2NoOX2p" => ImpactCause::PovertyAndHunger,
                    "rec5dt6EVyUeIaCR7" => ImpactCause::SeniorServices,
                    "recMt9349gwuRAQXf" => ImpactCause::JusticeAndEquity,
                    "rec8cH6YTQMeYqXUh" => ImpactCause::VeteransAndMilitaryFamilies,
                    _ => ImpactCause::Other,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Builder, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IntermediateVolunteerData {
    #[builder(setter(into))]
    pub first_name: String,
    #[builder(setter(into))]
    pub last_name: String,
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into), default = "None")]
    pub phone: Option<String>,
    #[builder(default = "Gender::PreferNotToSay")]
    #[serde(rename = "Gender")]
    pub volunteer_gender: Gender,
    #[builder(default = "vec![Ethnicity::PreferNotToSay]")]
    #[serde(rename = "Ethnicity")]
    pub volunteer_ethnicity: Vec<Ethnicity>,
    #[builder(default = "AgeRange::R18_24")]
    #[serde(rename = "AgeRange")]
    pub volunteer_age_range: AgeRange,
    #[builder(default = "vec![]")]
    pub university: Vec<String>,
    #[serde(rename = "LGBT")]
    pub lgbt: Lgbt,
    pub country: String,
    #[builder(setter(into), default = "None")]
    #[serde(rename = "State")]
    pub us_state: Option<String>,
    #[builder(default = "vec![Fli::PreferNotToSay]")]
    #[serde(rename = "FLI")]
    pub fli: Vec<Fli>,
    pub student_stage: StudentStage,
    pub majors: String,
    pub minors: Option<String>,
    pub hear_about: Vec<VolunteerHearAbout>,
}

impl From<IntermediateVolunteerData> for CreateVolunteer {
    fn from(value: IntermediateVolunteerData) -> Self {
        CreateVolunteer {
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            phone: value.phone,
            volunteer_gender: value.volunteer_gender,
            volunteer_ethnicity: value.volunteer_ethnicity,
            volunteer_age_range: value.volunteer_age_range,
            university: value.university,
            lgbt: value.lgbt,
            country: value.country,
            us_state: value.us_state,
            fli: value.fli,
            student_stage: value.student_stage,
            majors: value.majors.split(",").map(|m| m.trim().to_string()).collect(),
            minors: value
                .minors
                .unwrap_or_default()
                .split(",")
                .map(|m| m.trim().to_string())
                .collect(),
            hear_about: value.hear_about,
        }
    }
}

#[derive(Builder, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IntermediateMentorData {
    // pub project_cycle_id: Uuid,
    #[builder(setter(into))]
    pub first_name: String,
    #[builder(setter(into))]
    pub last_name: String,
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into))]
    pub phone: String,
    #[builder(setter(into))]
    pub company: String,
    #[builder(setter(into))]
    pub job_title: String,
    #[builder(setter(into))]
    pub country: String,
    #[builder(setter(into), default = "None")]
    pub us_state: Option<String>,
    #[builder(setter(into))]
    pub years_experience: String,
    #[builder(setter(into))]
    pub experience_level: MentorExperienceLevel,
    pub prior_mentorship: Vec<String>,
    #[serde(rename = "PriorDFG")]
    pub prior_dfg: Option<Vec<String>>,
    pub university: Option<Vec<String>>,
    pub hear_about: Option<Vec<VolunteerHearAbout>>,
}

impl From<IntermediateMentorData> for CreateMentor {
    fn from(value: IntermediateMentorData) -> Self {
        CreateMentor {
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            phone: value.phone,
            company: value.company,
            job_title: value.job_title,
            country: value.country,
            us_state: value.us_state,
            years_experience: match value.years_experience.as_str() {
                "2-5" => MentorYearsExperience::R2_5,
                "6-10" => MentorYearsExperience::R6_10,
                "11-15" => MentorYearsExperience::R11_15,
                "16-20" => MentorYearsExperience::R16_20,
                _ => MentorYearsExperience::R21Plus,
            },
            experience_level: value.experience_level,
            prior_mentor: value.prior_mentorship.contains(&"Yes, I've been a mentor".to_owned()),
            prior_mentee: value.prior_mentorship.contains(&"Yes, I've been a mentee".to_owned()),
            // prior_student: !value.prior_dfg.contains(&"No".to_owned()),
            prior_student: value.prior_dfg.unwrap_or_default().contains(&"Yes".to_owned()),
            university: value.university.unwrap_or_default(),
            hear_about: value.hear_about.unwrap_or(vec![VolunteerHearAbout::Other]),
        }
    }
}
