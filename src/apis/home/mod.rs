#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};

use crate::models::{
    feed::Feed,
    highlight_service::{Feature, FeatureStatus, HighlightService, WorkingFeature},
    models::Member,
    service::Service,
    slogan::Slogan,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetHomeResponse {
    pub slogan: Slogan,
    pub services: Vec<Service>,
    pub highlight_service: HighlightService,
    pub feeds: Vec<Feed>,
    pub members: Vec<Member>,
}

#[server(endpoint = "/home", input = GetUrl, output = Json)]
pub async fn get_home() -> Result<GetHomeResponse, ServerFnError> {
    tracing::debug!("GET /home");

    Ok(GetHomeResponse {
        slogan: Slogan {
            title: "Bringing Blockchain\nto Real-World Challenges".to_string(),
            description: "Biyard is a leading blockchain technology company focused on building decentralized solutions that drive innovation and transparency. From secure digital content rights protection to enhancing the transparency, trust, and efficiency of public polls and surveys, we empower governments, enterprises, and developers to unlock the full potential of Web3. Our flagship platform, d.AGIT, pioneers new ways to safeguard and manage digital assets with trust and security. At Biyard, we’re shaping the future of a decentralized digital economy.".to_string(),
        },
        services: vec![
            Service {
                category: "Art".to_string(),
                name: "d.AGIT".to_string(),
                short_description: "Service 1 description".to_string(),
                image: "https://cdn.pixabay.com/photo/2015/11/03/08/56/service-1019821_640.jpg".to_string(),
            },
            Service {
                category: "Art".to_string(),
                name: "d.AGIT".to_string(),
                short_description: "Service 1 description".to_string(),
                image: "https://cdn.pixabay.com/photo/2015/11/03/08/56/service-1019821_640.jpg".to_string(),
            },
            Service {
                category: "Art".to_string(),
                name: "d.AGIT".to_string(),
                short_description: "Service 1 description".to_string(),
                image: "https://cdn.pixabay.com/photo/2015/11/03/08/56/service-1019821_640.jpg".to_string(),
            },
        ],
        highlight_service: HighlightService {
            name: "d.AGIT".to_string(),
            description: "Experience full stack decentralization: from DAOs and crypto cloud services to games, NFTs, and social media, the Internet Computer has something for everyone.".to_string(),
            external_link: "https://dagit.club".to_string(),
            past_features: vec![
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::Completed("".to_string()),
                },
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::Completed("".to_string()),
                },
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::Completed("".to_string()),
                },
            ],
            future_features: vec![
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::NotStarted,
                },
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::NotStarted,
                },
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::NotStarted,
                },
            ],
            working_features: vec![
                WorkingFeature {
                    code_name: "Alpha 1".to_string(),
                    goal: "Alpha 1 aims to blah blah".to_string(),
                    status: FeatureStatus::Completed("Sep 24, 2024".to_string()),
                    features: vec![
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::Completed("Sep 24, 2024".to_string()),
                        },
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::Completed("Sep 24, 2024".to_string()),
                        },
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::Completed("Sep 24, 2024".to_string()),
                        },
                    ],
                },
                WorkingFeature {
                    code_name: "Alpha 2".to_string(),
                    goal: "Alpha 2 aims to blah blah".to_string(),
                    status: FeatureStatus::InProgress,
                    features: vec![
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::Completed("Sep 25, 2024".to_string()),
                        },
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::InProgress,
                        },
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::InProgress,
                        },
                    ],
                }
            ],
        },
        feeds: vec![
            Feed {
                title: "대한민국, 테라⋅루나라는 핑계에서 벗어나야할 시간".to_string(),
                published: "Sep. 25, 2024".to_string(),
                external_link: "http://www.opinionnews.co.kr/news/articleView.html?idxno=105514".to_string(),
            },
            Feed {
                title: "이제는 우리도 ‘종이없는 정부'가 필요, 디지털정부와 블록체인의 역할".to_string(),
                published: "Sep. 19, 2024".to_string(),
                external_link: "http://www.opinionnews.co.kr/news/articleView.html?idxno=105260".to_string(),
            },
            Feed {
                title: "예술과 블록체인, NFT 이후의 가능성".to_string(),
                published: "Sep. 11, 2024".to_string(),
                external_link: "http://www.opinionnews.co.kr/news/articleView.html?idxno=104897".to_string(),
            },
            Feed {
                title: "일본의 Web3 혁신과 도전, 대한민국이 놓치고 있는 것은?".to_string(),
                published: "Sep. 4, 2024".to_string(),
                external_link: "http://www.opinionnews.co.kr/news/articleView.html?idxno=104524".to_string(),
            },
        ],
        members: vec![
            Member {
                name: "Summer".to_string(),
                role: "Founder & CEO".to_string(),
                image: "https://dev.biyard.co/members/summer.png".to_string(),
                description: "Summer is".to_string(),
                email: Some("summer@biyard.co".to_string()),
                github: None,
                linkedin: None,
                web: None,
            },
            Member {
                name: "Miner".to_string(),
                role: "CEO & CTO".to_string(),
                image: "https://dev.biyard.co/members/miner.png".to_string(),
                description: "Summer is".to_string(),
                email: Some("miner@biyard.co".to_string()),
                github: Some("https://github.com/hackartists".to_string()),
                linkedin: None,
                web: None,
            },
            Member {
                name: "Rosa".to_string(),
                role: "Project Manager".to_string(),
                image: "https://dev.biyard.co/members/rosa.png".to_string(),
                description: "Senior Project Manager with over 10 years of experience in developing and managing projects for major corporations and national research institutes in South Korea. She holds a Bachelor's and Master's degree in Engineering from Korea University, as well as a Master's in Technology Management from KAIST.".to_string(),
                email: Some("rosa@biyard.co".to_string()),
                github: None,
                linkedin: None,
                web: None,
            },
            Member {
                name: "Victor".to_string(),
                role: "Software Engineer".to_string(),
                image: "https://dev.biyard.co/members/victor.png".to_string(),
                description: "".to_string(),
                email: Some("victor@biyard.co".to_string()),
                github: None,
                linkedin: None,
                web: None,
            },
            Member {
                name: "Ryan".to_string(),
                role: "Software Engineer".to_string(),
                image: "https://dev.biyard.co/members/victor.png".to_string(),
                description: "".to_string(),
                email: Some("ryan@biyard.co".to_string()),
                github: None,
                linkedin: None,
                web: None,
            },
            Member {
                name: "Jace".to_string(),
                role: "UX/UI Designer".to_string(),
                image: "https://dev.biyard.co/members/jace.png".to_string(),
                description: "".to_string(),
                email: Some("ryan@biyard.co".to_string()),
                github: None,
                linkedin: None,
                web: None,
            },
            Member {
                name: "Jihwan".to_string(),
                role: "Researcher".to_string(),
                image: "https://dev.biyard.co/members/victor.png".to_string(),
                description: "Ji-hwan is responsible for analyzing blockchain market trends and technologies. He also manages the company's investments and partnerships, contributing to strategic growth and collaboration.".to_string(),
                email: Some("jihwan@biyard.co".to_string()),
                github: None,
                linkedin: None,
                web: None,
            },
        ],
    })
}
