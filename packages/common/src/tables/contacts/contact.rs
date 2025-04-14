use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/contacts", table = contacts)]
pub struct Contact {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = submit)]
    pub last_name: String,
    #[api_model(action = submit)]
    pub first_name: String,
    #[api_model(action = submit, unique)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = submit)]
    pub company_name: String,
    #[api_model(action = submit, type = INTEGER)]
    pub needs: Need,
    #[api_model(action = submit)]
    pub help: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Need {
    #[default]
    #[translate(en = "General Inquiry", ko = "일반 문의")]
    GeneralInquiry = 1,
    #[translate(en = "Technical Support", ko = "기술 지원")]
    TechnicalSupport = 2,
    #[translate(en = "Partnership & Collaboration", ko = "제휴 및 협업")]
    PartnershipCollaboration = 3,
    #[translate(en = "Investment & Funding", ko = "투자 및 자금")]
    InvestmentFunding = 4,
    #[translate(en = "Regulatory & Legal Concerns", ko = "규제 및 법률 문제")]
    RegulatoryLegalConcerns = 5,
    #[translate(en = "Feedback & Suggestions", ko = "피드백 및 제안")]
    FeedbackSuggestions = 6,
}
