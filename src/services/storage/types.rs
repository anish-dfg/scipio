//! This module contains the definition of several types used in the storage layer.

use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

/// Possible project roles for a volunteer.
///
/// These are _not_ authentication roles, but rather roles that a volunteer can have in a project.
#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq, Display)]
#[serde(rename_all = "camelCase")]
pub enum VolunteerRole {
    /// An engineer on a project
    #[display("engineer")]
    Engineer,
    /// A designer on a project
    #[display("designer")]
    Designer,
    /// An engineering manager on a project
    #[display("engineering_manager")]
    EngineeringManager,
    /// A design manager on a project
    #[display("design_manager")]
    DesignManager,
    /// A product manager on a project
    #[display("product_manager")]
    ProductManager,
}

/// Possible states an asynchronous job can be in
#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "job_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum JobStatus {
    /// The job is pending
    ///
    /// This is the default state for a job at creation and indicates that there is work to be done.
    Pending,
    /// The job has terminated with an erro
    Error,
    /// The job has completed successfully
    Complete,
    /// The job has been cancelled
    Cancelled,
}

/// Possible destinations for exporting users
#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq, Display)]
#[serde(rename_all = "camelCase")]
pub enum ExportDesination {
    #[display("google_workspace")]
    GoogleWorkspace,
    #[display("okta")]
    Okta,
}

/// Possible types of jobs that can be run
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum JobType {
    /// Import a base from Airtable
    AirtableImportBase,
    /// Export users from Airtable to a valid export destination
    AirtableExportUsers,
    /// Undo an export of users to Workspace
    UndoWorkspaceExport,
}

/// Data needed to run a job
// TODO: Review how this is used and see if it can be merged with `JobType`. Also, consider adding
// new consider new fields to each variant depending on what information might be useful.
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum JobData {
    /// Data we track when we start a job to import a base from Airtable.
    AirtableImportBase {
        #[serde(rename = "baseId")]
        base_id: String,
    },
    /// Data we track when we start a job to export users from Airtable to a destination.
    AirtableExportUsers {
        #[serde(rename = "exportDestination")]
        export_destination: ExportDesination,
    },
    /// Data we track when we start a job to undo an export of users to Workspace.
    UndoWorkspaceExport { volunteers: Vec<(Uuid, String)> },
}

/// Details about a job
///
/// * `job_type`: The type of the job
/// * `error`: An error message if the job failed (otherwise this is `None`)
/// * `data`: Job metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobDetails {
    pub job_type: JobType,
    pub error: Option<String>,
    #[serde(flatten)]
    pub data: JobData,
}

/// Possible age ranges for volunteers
///
/// These are the age ranges that volunteers can select when signing up through Airtable.
#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "age_range")]
#[serde(rename_all = "camelCase")]
pub enum AgeRange {
    #[serde(rename = "18 - 24")]
    #[sqlx(rename = "18-24")]
    R18_24,
    #[serde(rename = "25 - 29")]
    #[sqlx(rename = "25-29")]
    R25_29,
    #[serde(rename = "30 - 34")]
    #[sqlx(rename = "30-34")]
    R30_34,
    #[serde(rename = "35 - 39")]
    #[sqlx(rename = "35-39")]
    R35_39,
    #[serde(rename = "40 - 44")]
    #[sqlx(rename = "40-44")]
    R40_44,
    #[serde(rename = "45 - 59")]
    #[sqlx(rename = "45-59")]
    R45_59,
    #[serde(rename = "60 - 64")]
    #[sqlx(rename = "60-64")]
    R60_64,
    #[serde(rename = "65+")]
    #[sqlx(rename = "65+")]
    ROver65,
}

/// Possible ethnicities for volunteers
///
/// These are the ethnicities that volunteers can select when signing up through Airtable.
#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "ethnicity", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum Ethnicity {
    #[serde(rename = "Asian")]
    Asian,
    #[serde(rename = "White or Caucasian")]
    WhiteOrCaucasian,
    #[serde(rename = "Black or African American")]
    BlackOrAfricanAmerican,
    #[serde(rename = "American Indian or Alaska Native")]
    AmericanIndianOrAlaskaNative,
    #[serde(rename = "Native Hawaiian or Pacific Islander")]
    NativeHawaiianOrPacificIslander,
    #[serde(rename = "Latino or Hispanic")]
    LatinoOrHispanic,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Prefer not to say")]
    PreferNotToSay,
}

/// Possible sizes for nonprofit clients
///
/// These are the sizes that nonprofits can select when requesting Develop for Good's services
/// through Airtable.
#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "client_size")]
#[serde(rename_all = "camelCase")]
pub enum ClientSize {
    #[serde(rename = "0")]
    #[sqlx(rename = "0")]
    S0,
    #[serde(rename = "1-5")]
    #[sqlx(rename = "1-5")]
    S1_5,
    #[serde(rename = "6-20")]
    #[sqlx(rename = "6-20")]
    S6_20,
    #[serde(rename = "21-50")]
    #[sqlx(rename = "21-50")]
    S21_50,
    #[serde(rename = "51-100")]
    #[sqlx(rename = "51-100")]
    S51_100,
    #[serde(rename = "101-500")]
    #[sqlx(rename = "101-500")]
    S101_500,
    #[serde(rename = "500+")]
    #[sqlx(rename = "500+")]
    SOver500,
}

/// Possible genders for volunteers
///
/// These are the possible genders that volunteers can select when signing up through Airtable.
#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "gender", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum Gender {
    #[serde(rename = "Woman")]
    Woman,
    #[serde(rename = "Man")]
    Man,
    #[serde(rename = "Non-binary / Non-conforming")]
    NonBinary,
    #[serde(rename = "Prefer to self-describe in another way")]
    Other,
    #[serde(rename = "Prefer not to say")]
    PreferNotToSay,
}

#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "lgbt_status", rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
pub enum Lgbt {
    Yes,
    No,
    #[serde(rename = "No, but I identify as an LGBTQ+ Ally")]
    Ally,
    #[serde(rename = "Prefer not to say")]
    PreferNotToSay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "student_stage", rename_all = "snake_case")]
pub enum StudentStage {
    Freshman,
    Sophomore,
    Junior,
    Senior,
    #[serde(rename = "Master's student")]
    MastersStudent,
    #[serde(rename = "PhD student")]
    PhdStudent,
    #[serde(rename = "Recent graduate")]
    RecentGraduate,
}

#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "nonprofit_hear_about", rename_all = "snake_case")]
pub enum NonprofitHearAbout {
    Linkedin,
    #[serde(rename = "Former Develop for Good client")]
    FormerClient,
    #[serde(rename = "Develop for Good member")]
    DfgMember,
    #[serde(rename = "Online ad")]
    OnlineAd,
    #[serde(rename = "News article")]
    NewsArticle,
    #[serde(rename = "Social media")]
    SocialMedia,
    #[serde(rename = "Company nonprofit network")]
    CompanyNonprofitNetwork,
    #[serde(rename = "Fast Forward")]
    FastForward,
    #[serde(rename = "All Stars Helping Kids")]
    AllStarsHelpingKids,
    #[serde(rename = "Word-of-mouth")]
    WordOfMouth,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "volunteer_hear_about", rename_all = "snake_case")]
pub enum VolunteerHearAbout {
    #[serde(rename = "Linkedin")]
    Linkedin,
    #[serde(rename = "From my university")]
    University,
    #[serde(rename = "From my company's social impact team")]
    CompanySocialImpactTeam,
    #[serde(rename = "From a colleague")]
    Colleague,
    #[serde(rename = "From a Develop for Good member")]
    DfgMember,
    #[serde(rename = "From a nonprofit")]
    Nonprofit,
    #[serde(rename = "Online ad")]
    OnlineAd,
    #[serde(rename = "Instagram")]
    Instagram,
    #[serde(rename = "Word of mouth")]
    WordOfMouth,
    #[serde(rename = "A bootcamp")]
    Bootcamp,
    #[serde(rename = "Discord or Slack group")]
    DiscordOrSlack,
    #[serde(rename = "I don't remember")]
    Unknown,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "impact_cause", rename_all = "snake_case")]
pub enum ImpactCause {
    Animals,
    #[serde(rename = "Career & professional development")]
    CareerAndProfessionalDevelopment,
    #[serde(rename = "Disaster relief")]
    DisasterRelief,
    #[serde(rename = "Education")]
    Education,
    #[serde(rename = "Environment & sustainability")]
    EnvironmentAndSustainability,
    #[serde(rename = "Faith & religion")]
    FaithAndReligion,
    #[serde(rename = "Health & medicine")]
    HealthAndMedicine,
    #[serde(rename = "Global relations")]
    GlobalRelations,
    #[serde(rename = "Poverty & hunger")]
    PovertyAndHunger,
    #[serde(rename = "Senior services")]
    SeniorServices,
    #[serde(rename = "Justice & equity")]
    JusticeAndEquity,
    #[serde(rename = "Veterans & military families")]
    VeteransAndMilitaryFamilies,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "mentor_experience_level", rename_all = "snake_case")]
pub enum MentorExperienceLevel {
    Intermediate,
    #[serde(rename = "First-level management")]
    FirstLevelManagement,
    #[serde(rename = "Middle management")]
    MiddleManagement,
    #[serde(rename = "Senior, executive, or top-level management")]
    SeniorOrExecutive,
}

#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "mentor_years_experience", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum MentorYearsExperience {
    #[sqlx(rename = "2-5")]
    #[serde(rename = "2-5")]
    R2_5,
    #[sqlx(rename = "6-10")]
    #[serde(rename = "6-10")]
    R6_10,
    #[sqlx(rename = "11-15")]
    #[serde(rename = "11-15")]
    R11_15,
    #[sqlx(rename = "16-20")]
    #[serde(rename = "16-20")]
    R16_20,
    #[sqlx(rename = "21+")]
    #[serde(rename = "21+")]
    R21Plus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "fli_status", rename_all = "snake_case")]
pub enum Fli {
    #[serde(rename = "First-generation")]
    FirstGeneration,
    #[serde(rename = "Low-income")]
    LowIncome,
    #[serde(rename = "Neither")]
    Neither,
    #[serde(rename = "Prefer not to say")]
    PreferNotToSay,
}
