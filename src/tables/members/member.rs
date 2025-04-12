use bdk::prelude::*;

#[api_model(base = "/api/members", table = members)]
pub struct Member {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub name: String,
    #[api_model(summary)]
    pub image: String,
    #[api_model(summary)]
    pub role: MemberRole,
    #[api_model(summary)]
    pub email: String,
    #[api_model(summary)]
    pub web: Option<String>,
    #[api_model(summary)]
    pub linkedin: Option<String>,
    #[api_model(summary)]
    pub github: Option<String>,
    #[api_model(summary)]
    pub description: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum MemberRole {
    #[default]
    #[translate(ko = "대표이사", en = "Founder & CEO")]
    Founder = 1,
    #[translate(ko = "CEO & CTO", en = "CEO & CTO")]
    CeoAndCto = 2,
    #[translate(ko = "Project Manager", en = "Project Manager")]
    PM = 3,
    #[translate(ko = "UI/UX 디자이너", en = "UI/UX Designer")]
    Designer = 4,
    #[translate(ko = "소프트웨어 엔지니어", en = "Software Engineer")]
    Developer = 5,
    #[translate(ko = "연구원", en = "Research")]
    Researcher = 6,
}
