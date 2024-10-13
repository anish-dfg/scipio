use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde_with::skip_serializing_none]
pub struct Address {
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into), default = "None")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde_with::skip_serializing_none]
pub struct Personalization {
    #[builder(setter(into), default = "None")]
    pub from: Option<Address>,
    pub to: Vec<Address>,
    #[builder(setter(into), default = "None")]
    pub cc: Option<Vec<Address>>,
    #[builder(setter(into), default = "None")]
    pub bcc: Option<Vec<Address>>,
    #[builder(setter(into), default = "None")]
    pub subject: Option<String>,
    #[builder(setter(into), default = "None")]
    pub headers: Option<Value>,
    #[builder(setter(into), default = "None")]
    pub substitutions: Option<Value>,
    #[builder(setter(into), default = "None")]
    pub dynamic_template_data: Option<Value>,
    #[builder(setter(into), default = "None")]
    pub custom_args: Option<Value>,
    #[builder(setter(into), default = "None")]
    pub send_at: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MailContentMime {
    #[serde(rename = "text/plain")]
    Plain,
    #[serde(rename = "text/html")]
    Html,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde_with::skip_serializing_none]
pub struct MailContent {
    pub value: String,
    #[serde(rename = "type")]
    pub mime_type: MailContentMime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AttachmentDisposition {
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "inline")]
    Inline,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde_with::skip_serializing_none]
pub struct Attachment {
    pub content: String,
    pub filename: String,
    #[serde(rename = "type")]
    pub mime_type: String,
    #[builder(setter(into), default = "None")]
    pub disposition: Option<AttachmentDisposition>,
    #[builder(setter(into), default = "None")]
    pub content_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde_with::skip_serializing_none]
pub struct Asm {
    group_id: u64,
    #[builder(setter(into), default = "None")]
    groups_to_display: Option<Vec<u64>>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
pub struct MailSettingEnable {
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
pub struct FooterSetting {
    pub enable: bool,
    #[builder(setter(into), default = "None")]
    pub text: Option<String>,
    #[builder(setter(into), default = "None")]
    pub html: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde_with::skip_serializing_none]
pub struct MailSettings {
    #[builder(setter(into), default = "None")]
    pub bypass_list_management: Option<MailSettingEnable>,
    #[builder(setter(into), default = "None")]
    pub bypass_spam_management: Option<MailSettingEnable>,
    #[builder(setter(into), default = "None")]
    pub bypass_bounce_management: Option<MailSettingEnable>,
    #[builder(setter(into), default = "None")]
    pub bypass_unsubscribe_management: Option<MailSettingEnable>,
    #[builder(setter(into), default = "None")]
    pub footer: Option<FooterSetting>,
    #[builder(setter(into), default = "None")]
    pub sandbox_mode: Option<MailSettingEnable>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
pub struct TrackingSettingsClickTracking {
    enable: bool,
    enable_text: bool,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde_with::skip_serializing_none]
pub struct TrackingSettingsOpenTracking {
    enable: bool,
    #[builder(setter(into), default = "None")]
    substitution_tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde_with::skip_serializing_none]
pub struct TrackingSettingsSubscriptionTracking {
    enable: bool,
    #[builder(setter(into), default = "None")]
    text: Option<String>,
    #[builder(setter(into), default = "None")]
    html: Option<String>,
    #[builder(setter(into), default = "None")]
    substitution_tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde_with::skip_serializing_none]
pub struct TrackingSettingsGAnalytics {
    enable: bool,
    #[builder(setter(into), default = "None")]
    utm_source: Option<String>,
    #[builder(setter(into), default = "None")]
    utm_medium: Option<String>,
    #[builder(setter(into), default = "None")]
    utm_term: Option<String>,
    #[builder(setter(into), default = "None")]
    utm_content: Option<String>,
    #[builder(setter(into), default = "None")]
    utm_campaign: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde_with::skip_serializing_none]
pub struct TrackingSettings {
    #[builder(setter(into), default = "None")]
    click_tracking: Option<TrackingSettingsClickTracking>,
    #[builder(setter(into), default = "None")]
    open_tracking: Option<TrackingSettingsOpenTracking>,
    #[builder(setter(into), default = "None")]
    subscription_tracking: Option<TrackingSettingsSubscriptionTracking>,
    #[builder(setter(into), default = "None")]
    ganalytics: Option<TrackingSettingsGAnalytics>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde_with::skip_serializing_none]
pub struct Mail {
    pub personalizations: Vec<Personalization>,
    pub from: Address,
    #[builder(setter(into), default = "None")]
    pub reply_to: Option<Address>,
    #[builder(setter(into), default = "None")]
    pub reply_to_list: Option<Vec<Address>>,
    #[builder(setter(into))]
    pub subject: String,
    pub content: Vec<MailContent>,
    #[builder(setter(into), default = "None")]
    pub attachments: Option<Vec<Attachment>>,
    #[builder(setter(into), default = "None")]
    pub template_id: Option<String>,
    #[builder(setter(into), default = "None")]
    pub headers: Option<Value>,
    #[builder(setter(into), default = "None")]
    pub categories: Option<Vec<String>>,
    #[builder(setter(into), default = "None")]
    pub custom_args: Option<String>,
    #[builder(setter(into), default = "None")]
    pub send_at: Option<u64>,
    #[builder(setter(into), default = "None")]
    pub batch_id: Option<String>,
    #[builder(setter(into), default = "None")]
    pub asm: Option<Asm>,
    #[builder(setter(into), default = "None")]
    pub ip_pool_name: Option<String>,
    #[builder(setter(into), default = "None")]
    pub mail_settings: Option<MailSettings>,
    #[builder(setter(into), default = "None")]
    pub tracking_settings: Option<TrackingSettings>,
}

impl TryFrom<Mail> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(value: Mail) -> std::result::Result<Self, Self::Error> {
        Ok(serde_json::to_vec(&value)?)
    }
}
