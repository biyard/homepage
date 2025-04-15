use bdk::prelude::*;

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum Info {
    #[translate(
        ko = "연락처 제출을 완료했습니다.",
        en = "Contact submission completed. Please check your email for further instructions."
    )]
    ContactSubmit,
    #[translate(
        ko = "업데이트를 받아보실수 있습니다.",
        en = "You can receive updates."
    )]
    KeepUpdateSubmit,
}
