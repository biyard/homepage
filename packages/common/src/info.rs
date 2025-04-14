use bdk::prelude::*;

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum Info {
    #[translate(
        ko = "연락처 제출을 완료했습니다.",
        en = "Contact submission completed. Please check your email for further instructions."
    )]
    ContactSubmit,
}
