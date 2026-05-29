#[derive(Clone)]
pub struct LocalizedStr {
    pub ko: &'static str,
    pub en: &'static str,
}

impl LocalizedStr {
    pub fn get(&self, lang: &str) -> &'static str {
        if lang == "en" { self.en } else { self.ko }
    }
}

macro_rules! ls {
    ($ko:expr, $en:expr) => {
        LocalizedStr { ko: $ko, en: $en }
    };
}

#[derive(Clone)]
pub struct ContactLabels {
    pub email: LocalizedStr,
    pub message: LocalizedStr,
    pub message_placeholder: LocalizedStr,
    pub privacy: LocalizedStr,
    pub submit: LocalizedStr,
}

#[derive(Clone)]
pub struct SiteContent {
    pub title: LocalizedStr,
    pub description: LocalizedStr,
    pub logo_src: &'static str,
    pub logo_alt: &'static str,
    pub favicon_src: &'static str,
    pub hero: Hero,
    pub ecosystem: Ecosystem,
    pub standard: Standard,
    pub use_cases_title: LocalizedStr,
    pub use_cases_lead: LocalizedStr,
    pub use_cases: Vec<UseCase>,
    pub product_gateway: ProductGateway,
    pub trust_proof: TrustProof,
    pub final_cta: FinalCta,
    pub contact_labels: ContactLabels,
    pub footer_columns: Vec<FooterColumn>,
    pub footer_text: LocalizedStr,
    pub footer_note: &'static str,
}

#[derive(Clone)]
pub struct Hero {
    pub title: LocalizedStr,
    pub sublead: LocalizedStr,
    pub lead: LocalizedStr,
    pub primary_cta: Cta,
    pub secondary_cta: Cta,
    pub proof_label: &'static str,
}

#[derive(Clone)]
pub struct Cta {
    pub label: LocalizedStr,
    pub href: &'static str,
}

#[derive(Clone)]
pub struct Standard {
    pub eyebrow: &'static str,
    pub title: LocalizedStr,
    pub body: LocalizedStr,
    pub source_label: &'static str,
    pub source_standard: &'static str,
    pub source_main_title: &'static str,
    pub source_subtitle_prefix: &'static str,
    pub source_subtitle_highlight: &'static str,
    pub source_url: &'static str,
    pub source_link_label: LocalizedStr,
    pub image_src: &'static str,
    pub image_alt: LocalizedStr,
}

#[derive(Clone)]
pub struct UseCase {
    pub title: LocalizedStr,
    pub detail: LocalizedStr,
    pub class_name: &'static str,
    pub tone_class: &'static str,
    pub icon: &'static str,
}

#[derive(Clone)]
pub struct ProductGateway {
    pub eyebrow: &'static str,
    pub title: LocalizedStr,
    pub cards: Vec<ProductCard>,
}

#[derive(Clone)]
pub struct ProductCard {
    pub name: &'static str,
    pub href: &'static str,
    pub cta_label: LocalizedStr,
    pub tone_class: &'static str,
    pub image_src: &'static str,
    pub image_alt: LocalizedStr,
    pub external: bool,
}

#[derive(Clone)]
pub struct Ecosystem {
    pub eyebrow: &'static str,
    pub title: &'static str,
    pub lead: &'static str,
    pub items: Vec<EcosystemItem>,
}

#[derive(Clone)]
pub struct EcosystemItem {
    pub logo_src: Option<&'static str>,
}

impl EcosystemItem {
    pub fn logo_value(&self) -> &'static str {
        self.logo_src.unwrap_or("")
    }
}

#[derive(Clone)]
pub struct TrustProof {
    pub eyebrow: &'static str,
    pub headline: &'static str,
    pub lead: LocalizedStr,
    pub milestones: Vec<ProofMilestone>,
    pub proof_items: Vec<ProofItem>,
}

#[derive(Clone)]
pub struct ProofMilestone {
    pub phase: &'static str,
    pub title: LocalizedStr,
    pub body: LocalizedStr,
}

#[derive(Clone)]
pub struct ProofItem {
    pub label: &'static str,
    pub title: LocalizedStr,
    pub detail: LocalizedStr,
    pub tone_class: &'static str,
}

#[derive(Clone)]
pub struct FinalCta {
    pub eyebrow: &'static str,
    pub title: LocalizedStr,
    pub body: LocalizedStr,
    pub primary: Cta,
    pub secondary: Cta,
    pub tertiary: Cta,
}

#[derive(Clone)]
pub struct FooterColumn {
    pub title: &'static str,
    pub links: Vec<FooterLink>,
}

#[derive(Clone)]
pub struct FooterLink {
    pub label: &'static str,
    pub href: &'static str,
}

#[derive(Clone)]
pub struct ProductLanding {
    pub theme_class: &'static str,
    pub title: LocalizedStr,
    pub description: LocalizedStr,
    pub product_name: LocalizedStr,
    pub eyebrow: LocalizedStr,
    pub hero_title: LocalizedStr,
    pub hero_lead: LocalizedStr,
    pub hero_note: LocalizedStr,
    pub primary_cta: Cta,
    pub secondary_cta: Cta,
    pub stats: Vec<Metric>,
    pub problem_title: LocalizedStr,
    pub problem_body: LocalizedStr,
    pub problem_callout_badge: LocalizedStr,
    pub problem_callout_body: LocalizedStr,
    pub problem_summary: LocalizedStr,
    pub problems: Vec<Feature>,
    pub why_title: LocalizedStr,
    pub why_lead: LocalizedStr,
    pub answer_eyebrow: LocalizedStr,
    pub answer_title: LocalizedStr,
    pub answer_body: LocalizedStr,
    pub answer_cards: Vec<Feature>,
    pub workflow_eyebrow: LocalizedStr,
    pub workflow_title: LocalizedStr,
    pub workflow_body: LocalizedStr,
    pub workflow: Vec<ProcessItem>,
    pub operations_eyebrow: LocalizedStr,
    pub operations_title: LocalizedStr,
    pub operations_body: LocalizedStr,
    pub operations_demo_label: LocalizedStr,
    pub operations: Vec<Feature>,
    pub proof_eyebrow: LocalizedStr,
    pub proof_title: LocalizedStr,
    pub proof_body: LocalizedStr,
    pub proof_points: Vec<Feature>,
    pub cta_eyebrow: LocalizedStr,
    pub cta_title: LocalizedStr,
    pub cta_body: LocalizedStr,
    pub related_label: LocalizedStr,
    pub related_name: &'static str,
    pub related_body: LocalizedStr,
    pub related_href: &'static str,
    pub related_link_label: LocalizedStr,
    // zkPoRL specific
    pub flow_section_title: LocalizedStr,
    pub flow_subtitle: LocalizedStr,
    pub flow_in1_title: LocalizedStr,
    pub flow_in2_title: LocalizedStr,
    pub flow_in3_title: LocalizedStr,
    pub flow_out1_title: LocalizedStr,
    pub flow_out1_sub: LocalizedStr,
    pub flow_out2_title: LocalizedStr,
    pub flow_out2_sub: LocalizedStr,
    pub flow_out3_title: LocalizedStr,
    pub flow_server_desc: LocalizedStr,
    pub flow_note: LocalizedStr,
    pub table_header1: LocalizedStr,
    pub table_header2: LocalizedStr,
    pub table_row1_label: LocalizedStr,
    pub table_row1_traditional: LocalizedStr,
    pub table_row1_new: LocalizedStr,
    pub table_row2_label: LocalizedStr,
    pub table_row2_traditional: LocalizedStr,
    pub table_row2_new: LocalizedStr,
    pub table_row3_label: LocalizedStr,
    pub table_row3_traditional: LocalizedStr,
    pub table_row3_new: LocalizedStr,
    // zkWallet specific
    pub mpc_asis_title: LocalizedStr,
    pub mpc_asis_item1: LocalizedStr,
    pub mpc_asis_item2: LocalizedStr,
    pub mpc_asis_item3: LocalizedStr,
    pub mpc_tobe_title: LocalizedStr,
    pub mpc_tobe_item1: LocalizedStr,
    pub mpc_tobe_item2: LocalizedStr,
    pub mpc_tobe_item3: LocalizedStr,
    pub mpc_section_title: LocalizedStr,
    pub mpc_section_lead: LocalizedStr,
    pub mpc_shard_label: LocalizedStr,
    pub mpc_partial_sig: LocalizedStr,
    pub mpc_collect_label: LocalizedStr,
    pub mpc_chain_label: LocalizedStr,
    pub mpc_note: LocalizedStr,
}

#[derive(Clone)]
pub struct Metric {
    pub value: LocalizedStr,
    pub label: LocalizedStr,
    pub detail: LocalizedStr,
}

#[derive(Clone)]
pub struct Feature {
    pub title: LocalizedStr,
    pub body: LocalizedStr,
}

#[derive(Clone)]
pub struct ProcessItem {
    pub number: &'static str,
    pub title: LocalizedStr,
    pub body: LocalizedStr,
}

pub fn site_content() -> SiteContent {
    SiteContent {
        title: ls!(
            "ZKRYPTO | ISO/IEC 27565:2026 표준 기반 ZKP 신뢰 레이어",
            "ZKRYPTO | ZKP Trust Layer Based on ISO/IEC 27565:2026"
        ),
        description: ls!(
            "지크립토는 ISO/IEC 27565:2026 글로벌 표준 기반 ZKP 기술로 데이터 공개를 최소화하고 필요한 사실만 증명하는 프라이버시 검증 구조를 설계합니다.",
            "ZKRYPTO designs privacy verification structures using ZKP technology based on the ISO/IEC 27565:2026 global standard — minimizing data disclosure and proving only what is necessary."
        ),
        logo_src: "/assets/zkrypto_logo.png",
        logo_alt: "ZKRYPTO",
        favicon_src: "/assets/zkrypto-favicon.png",
        hero: Hero {
            title: ls!(
                "프라이버시와 컴플라이언스를\nZKP로 연결합니다.",
                "Connecting Privacy and Compliance\nwith ZKP."
            ),
            sublead: ls!(
                "ZKP(Zero-knowledge Proofs)는 ISO/IEC에서도 표준화한 암호학적 체계입니다. ",
                "ZKP (Zero-knowledge Proofs) is a cryptographic scheme standardized by ISO/IEC."
            ),
            lead: ls!(
                "지크립토는 ZKP를 활용하여 정보를 직접 공개하지 않고도, 필요한 사실만 증명할 수 있는 신뢰 레이어를 제공합니다.",
                "ZKRYPTO provides a trust layer using ZKP — proving only what needs to be verified without directly exposing the underlying information."
            ),
            primary_cta: Cta { label: ls!("ZKP Product", "ZKP Product"), href: "#products" },
            secondary_cta: Cta { label: ls!("Contact", "Contact"), href: "#contact" },
            proof_label: "",
        },
        ecosystem: Ecosystem {
            eyebrow: "",
            title: "",
            lead: "Partners",
            items: vec![
                EcosystemItem { logo_src: Some("/assets/zkrypto_logo.png") },
                EcosystemItem { logo_src: Some("/assets/zkrypto_logo.png") },
                EcosystemItem { logo_src: Some("/assets/zkrypto_logo.png") },
                EcosystemItem { logo_src: Some("/assets/zkrypto_logo.png") },
                EcosystemItem { logo_src: Some("/assets/zkrypto_logo.png") },
                EcosystemItem { logo_src: Some("/assets/zkrypto_logo.png") },
                EcosystemItem { logo_src: Some("/assets/zkrypto_logo.png") },
                EcosystemItem { logo_src: Some("/assets/zkrypto_logo.png") },
            ],
        },
        use_cases_title: ls!(
            "공개하기 어려운 민감정보를 검증 받아야 하는 분야",
            "Fields that need to verify sensitive information without exposing it"
        ),
        use_cases_lead: ls!(
            "ZKP는 신원, 자격, 금융, 공공, AI 분야처럼 공개적으로 확인은 해야 하지만 원본은 공개하기가 어려운 딜레마를 해결합니다.",
            "ZKP resolves the dilemma in fields like identity, credentials, finance, public services, and AI — where verification is required but the underlying data cannot be disclosed."
        ),
        contact_labels: ContactLabels {
            email: ls!("이메일", "Email"),
            message: ls!("문의 내용", "Message"),
            message_placeholder: ls!("적용 분야, 검증하려는 업무, 상담 희망 내용을 남겨주세요.", "Please describe your use case, the process you'd like to verify, and any consultation topics."),
            privacy: ls!("답변을 위한 개인정보 수집·이용에 동의합니다.", "I agree to the collection and use of personal information for the purpose of responding to my inquiry."),
            submit: ls!("문의하기", "Send"),
        },
        standard: Standard {
            eyebrow: "Global Standard",
            title: ls!(
                "지크립토는 ISO/IEC 27565:2026과 함께 글로벌을 선도합니다.",
                "ZKRYPTO leads globally with ISO/IEC 27565:2026."
            ),
            body: ls!(
                "생년월일을 몰라도 성인 여부를 확인 가능하고, 거래내역 전체를 몰라도 거래 유효성을 검증할 수 있습니다.\nISO/IEC 27565:2026은 이러한 방식의 프라이버시 강화 기술을 다루는 글로벌 표준입니다.",
                "You can verify adulthood without knowing a birthdate, and validate a transaction without seeing the full transaction history.\nISO/IEC 27565:2026 is the global standard covering privacy-enhancing technologies of this kind."
            ),
            source_label: "",
            source_standard: "ISO/IEC 27565:2026",
            source_main_title: "Information security, cybersecurity and privacy protection",
            source_subtitle_prefix: "- Guidelines on privacy preservation based on",
            source_subtitle_highlight: "  Zero-Knowledge Proofs",
            source_url: "https://www.iso.org/standard/80398.html",
            source_link_label: ls!("자료: ISO 공식 사이트", "Source: ISO official site"),
            image_src: "/assets/iso-standard-dossier.png",
            image_alt: ls!(
                "ISO 표준 문서를 연상시키는 ZKP 프라이버시 검증 자료 이미지",
                "ZKP privacy verification reference image resembling an ISO standard document"
            ),
        },
        use_cases: vec![
            UseCase {
                title: ls!("가상자산 준법·감사", "Crypto Compliance & Audit"),
                detail: ls!("잔고대사, 감사 대응 증적자료", "Balance reconciliation, audit evidence"),
                class_name: "orbit-top", tone_class: "case-finance case-asset", icon: "audit",
            },
            UseCase {
                title: ls!("투표·공공 신뢰", "Voting & Public Trust"),
                detail: ls!("절차와 결과의 공정성 확보", "Ensuring fairness in process and results"),
                class_name: "orbit-right-top", tone_class: "case-trust", icon: "vote",
            },
            UseCase {
                title: ls!("디지털 신원", "Digital Identity"),
                detail: ls!("필요한 정보만 선택적 공개", "Selective disclosure of necessary information"),
                class_name: "orbit-right-bottom", tone_class: "case-trust", icon: "id",
            },
            UseCase {
                title: ls!("자격·연령 확인", "Credential & Age Verification"),
                detail: ls!("기준 충족 여부만 확인", "Verify compliance without revealing details"),
                class_name: "orbit-bottom", tone_class: "case-trust", icon: "age",
            },
            UseCase {
                title: ls!("공정추첨·경매", "Fair Draw & Auction"),
                detail: ls!("규칙대로 했는지 결과 검증", "Verify that rules were followed"),
                class_name: "orbit-left-bottom", tone_class: "case-finance case-lottery", icon: "lottery",
            },
            UseCase {
                title: ls!("CBDC·기관형 분산원장", "CBDC & Institutional DLT"),
                detail: ls!("정보보호 송금, 준비금 검증", "Privacy-preserving transfers, reserve verification"),
                class_name: "orbit-left-top", tone_class: "case-finance", icon: "cbdc",
            },
        ],
        product_gateway: ProductGateway {
            eyebrow: "ZKP Product",
            title: ls!(
                "검증 가능한 신뢰 레이어를 제공합니다.",
                "We provide a verifiable trust layer."
            ),
            cards: vec![
                ProductCard {
                    name: "zkWallet",
                    href: "/products/zkwallet",
                    cta_label: ls!("더 보기", "Learn more"),
                    tone_class: "product-card-wallet",
                    image_src: "/assets/zkwallet-product-slide-1.png",
                    image_alt: ls!("zkWallet 제품소개서 첫 페이지 이미지", "zkWallet product introduction"),
                    external: false,
                },
                ProductCard {
                    name: "zkPoRL",
                    href: "/products/zkporl",
                    cta_label: ls!("더 보기", "Learn more"),
                    tone_class: "product-card-porl",
                    image_src: "/assets/zkporl-product-slide-1.png",
                    image_alt: ls!("zkPoRL 제품소개서 첫 페이지 이미지", "zkPoRL product introduction"),
                    external: false,
                },
                ProductCard {
                    name: "zkVoting",
                    href: "/products/zkvoting",
                    cta_label: ls!("더 보기", "Learn more"),
                    tone_class: "product-card-voting",
                    image_src: "/assets/zkvoting-product-slide-2.png",
                    image_alt: ls!("zkVoting 제품소개서 스타일 초안 이미지", "zkVoting product introduction draft"),
                    external: true,
                },
            ],
        },
        trust_proof: TrustProof {
            eyebrow: "Trust",
            headline: "ZKRYPTO History",
            lead: ls!(
                "ZKP 기술 기반 신뢰 레이어를 설계합니다.",
                "We design a trust layer based on ZKP technology."
            ),
            milestones: vec![
                ProofMilestone {
                    phase: "Research",
                    title: ls!("ZKP 연구 기반 역량 축적", "Building ZKP Research Capabilities"),
                    body: ls!(
                        "프라이버시를 지키면서 사실을 검증하는 암호 기술을 연구해왔습니다.",
                        "We have researched cryptographic technology that verifies facts while preserving privacy."
                    ),
                },
                ProofMilestone {
                    phase: "Standard",
                    title: ls!("ISO/IEC 27565:2026 ZKP 기반 서비스", "ISO/IEC 27565:2026-Based Services"),
                    body: ls!(
                        "데이터 최소화, 선택적 공개의 원칙을 서비스에 구현했습니다.",
                        "We have implemented the principles of data minimization and selective disclosure in our services."
                    ),
                },
                ProofMilestone {
                    phase: "Product",
                    title: ls!("Fineapple 신뢰 인프라 설계", "Fineapple Trust Infrastructure Design"),
                    body: ls!(
                        "디지털 자산의 발행, 잔고증명에서 감사까지 설명 가능한 흐름으로 연결합니다.",
                        "Connecting digital asset issuance, proof of reserve, and audit in an explainable end-to-end flow."
                    ),
                },
                ProofMilestone {
                    phase: "Expansion",
                    title: ls!("공공·금융 적용 시나리오 확장", "Expanding to Public & Financial Sectors"),
                    body: ls!(
                        "신원, 자격, 투표, CBDC 등 적용 가능 영역을 구체화하고 있습니다.",
                        "We are defining applicable domains including identity, credentials, voting, and CBDC."
                    ),
                },
            ],
            proof_items: vec![
                ProofItem {
                    label: "Public",
                    title: ls!("CES 2024 최고혁신상 수상", "CES 2024 Best of Innovation Award"),
                    detail: ls!(
                        "zkVoting Poll Station, Cybersecurity & Personal Privacy 분야 Best of Innovation.",
                        "zkVoting Poll Station, Best of Innovation in Cybersecurity & Personal Privacy."
                    ),
                    tone_class: "proof-public",
                },
                ProofItem {
                    label: "Award",
                    title: ls!("CES 2023·2024 2년 연속 최고혁신상", "CES 2023·2024 Back-to-Back Best of Innovation"),
                    detail: ls!(
                        "zkVoting, zkWallet, zkVoting Poll Station 관련 수상 이력.",
                        "Award history for zkVoting, zkWallet, and zkVoting Poll Station."
                    ),
                    tone_class: "proof-award",
                },
                ProofItem {
                    label: "IP",
                    title: ls!("특허 11건 등록, 17건 출원", "11 Patents Granted, 17 Filed"),
                    detail: ls!(
                        "국내 4건, 해외 7건 등록으로 정리된 ZKP R&D·IP 포트폴리오.",
                        "A structured ZKP R&D and IP portfolio with 4 domestic and 7 international patents granted."
                    ),
                    tone_class: "proof-ip",
                },
                ProofItem {
                    label: "Research",
                    title: ls!("최근 5년 18개 논문 게재", "18 Papers Published in the Last 5 Years"),
                    detail: ls!(
                        "zkMarket, zkAML, Aegis, vCNN, zkLogis, zkVoting, Azeroth 등 ZKP 연구 성과.",
                        "ZKP research results including zkMarket, zkAML, Aegis, vCNN, zkLogis, zkVoting, and Azeroth."
                    ),
                    tone_class: "proof-research",
                },
                ProofItem {
                    label: "CBDC",
                    title: ls!("한국은행 CBDC 파일럿 프로젝트 수행", "Bank of Korea CBDC Pilot Project"),
                    detail: ls!(
                        "프라이버시 보호 CBDC, AML/CFT, 성능 검증 관련 이력.",
                        "Track record in privacy-preserving CBDC, AML/CFT, and performance validation."
                    ),
                    tone_class: "proof-public",
                },
                ProofItem {
                    label: "Public Trust",
                    title: ls!("중앙선거관리위원회 블록체인 투표 적용", "NEC Blockchain Voting Deployment"),
                    detail: ls!(
                        "zkVoting Poll Station 기반 공공 투표 적용 이력.",
                        "Track record of deploying zkVoting Poll Station for public elections."
                    ),
                    tone_class: "proof-public",
                },
                ProofItem {
                    label: "Certification",
                    title: ls!("ISO 9001·KISA 신속확인", "ISO 9001 & KISA Certification"),
                    detail: ls!(
                        "품질·보안 신뢰 인프라와 제품 적용 가능성을 보여주는 대외 증빙 항목입니다.",
                        "External certifications demonstrating quality, security trust infrastructure, and product applicability."
                    ),
                    tone_class: "proof-cert",
                },
                ProofItem {
                    label: "Award",
                    title: ls!("과기정통부장관상·모바일 기술대상", "MSIT Minister Award & Mobile Tech Award"),
                    detail: ls!(
                        "기술성과 제품 경쟁력을 함께 보여주는 수상 이력입니다.",
                        "Awards demonstrating both technical achievement and product competitiveness."
                    ),
                    tone_class: "proof-award",
                },
            ],
        },
        final_cta: FinalCta {
            eyebrow: "Next",
            title: ls!(
                "기존 서비스에 신뢰 레이어를 추가하세요.",
                "Add a trust layer to your existing services."
            ),
            body: ls!(
                "디지털 자산, 공공 신뢰인프라, 연구 협업 등",
                "Digital assets, public trust infrastructure, research collaboration, and more."
            ),
            primary: Cta { label: ls!("제품 보기", "View Products"), href: "#products" },
            secondary: Cta { label: ls!("문의 하기", "Contact Us"), href: "#contact" },
            tertiary: Cta { label: ls!("자료 요청", "Request Materials"), href: "#contact" },
        },
        footer_columns: vec![
            FooterColumn {
                title: "Trusted Layer",
                links: vec![
                    FooterLink { label: "ZKP Product", href: "/#products" },
                    FooterLink { label: "zkPoRL", href: "/products/zkporl" },
                    FooterLink { label: "zkWallet", href: "/products/zkwallet" },
                    FooterLink { label: "zkVoting", href: "https://www.zkvoting.com" },
                ],
            },
            FooterColumn {
                title: "Trust",
                links: vec![
                    FooterLink { label: "Ecosystem", href: "/#ecosystem" },
                    FooterLink { label: "Standard", href: "/#standard" },
                    FooterLink { label: "Proof", href: "/#proof" },
                    FooterLink { label: "Use Cases", href: "/#use-cases" },
                ],
            },
            FooterColumn {
                title: "Contact",
                links: vec![
                    FooterLink { label: "Contact", href: "/#contact" },
                    FooterLink { label: "Product", href: "/#product" },
                    FooterLink { label: "Home", href: "/#top" },
                ],
            },
        ],
        footer_text: ls!(
            "표준 기반 ZKP로 디지털 자산과 공공 데이터를 위한 신뢰 검증 인프라를 구축합니다.",
            "Building trust verification infrastructure for digital assets and public data using standard-based ZKP."
        ),
        footer_note: "ISO/IEC 27565:2026",
    }
}

pub fn product_landing(slug: &str) -> Option<ProductLanding> {
    match slug {
        "zkporl" => Some(zkporl_landing()),
        "zkwallet" => Some(zkwallet_landing()),
        "zkvoting" => Some(zkvoting_landing()),
        _ => None,
    }
}

fn zkporl_landing() -> ProductLanding {
    ProductLanding {
        theme_class: "product-theme-porl",
        title: ls!(
            "zkPoRL | Fineapple 고객부채·준비자산 검증 인프라",
            "zkPoRL | Fineapple Proof of Reserve & Liability Verification"
        ),
        description: ls!(
            "zkPoRL은 거래소, 수탁사, 은행이 고객부채와 준비자산의 정합성을 더 짧은 주기로 확인하도록 돕는 검증 인프라입니다.",
            "zkPoRL is a verification infrastructure that helps exchanges, custodians, and banks reconcile customer liabilities and reserve assets on shorter cycles."
        ),
        product_name: ls!("zkPoRL", "zkPoRL"),
        eyebrow: ls!("Fineapple 금융 제품군", "Fineapple Financial Suite"),
        hero_title: ls!(
            "고객부채와 준비자산을 짧은 주기로 맞춰 보는 검증 인프라",
            "Verification Infrastructure for Continuous Reserve-Liability Reconciliation"
        ),
        hero_lead: ls!(
            "개별 잔고를 공개하지 않고도, 원장과 자산 상태가 맞는지 확인할 수 있게 돕습니다.",
            "Verify that ledger and asset states are consistent — without exposing individual balances."
        ),
        hero_note: ls!(" ", " "),
        primary_cta: Cta { label: ls!("도입 상담하기", "Request a Consultation"), href: "/#contact" },
        secondary_cta: Cta { label: ls!("제품군으로 돌아가기", "Back to Products"), href: "/#products" },
        stats: vec![
            Metric {
                value: ls!("Privacy", "Privacy"),
                label: ls!("정보보호", ""),
                detail: ls!(
                    "민감정보를 외부에 직접 노출하지 않는 검증 구조",
                    "Verification structure that never directly exposes sensitive data"
                ),
            },
            Metric {
                value: ls!("Verifiability", "Verifiability"),
                label: ls!("검증", ""),
                detail: ls!(
                    "정의된 입력과 검증 명제의 수학적 정합성 확인",
                    "Mathematical consistency check of defined inputs and verification propositions"
                ),
            },
            Metric {
                value: ls!("Operations", "Operations"),
                label: ls!("운영", ""),
                detail: ls!(
                    "증명 결과를 운영 대시보드, 로그, 증적 자료로 연결",
                    "Connecting proof results to operational dashboards, logs, and audit evidence"
                ),
            },
        ],
        problem_title: ls!(
            "스냅샷과 후행점검만으로는 운영 중 리스크를 설명하기 어렵습니다",
            "Snapshots and lagging reviews alone cannot explain operational risk"
        ),
        problem_body: ls!(
            "정기 감사와 특정 시점 리포트는 필요하지만, 점검 사이에 발생하는 장부 불일치, 기준시점 차이, 누락·중복 이벤트를 빠르게 포착하기 어렵습니다.",
            "Periodic audits and point-in-time reports are necessary, but it is difficult to quickly detect ledger discrepancies, timestamp mismatches, or missing and duplicate events between checks."
        ),
        problem_callout_badge: ls!("기존 방식의 한계", "Limitations of Existing Approaches"),
        problem_callout_body: ls!(
            "점검 사이에 발생한 장부 불일치와 이상징후를 조기에 포착하기 어렵습니다.",
            "Ledger discrepancies and anomalies occurring between checks are difficult to detect early."
        ),
        problem_summary: ls!(
            "문제는 \"자산이 충분하다\"만이 아니라, 운영 중 \"변동성을 설명할 수 있느냐\"입니다.",
            "The question is not just \"are assets sufficient\" — it is \"can you explain volatility during operations\"."
        ),
        problems: vec![
            Feature { title: ls!("대사 주기", "Reconciliation Cycle"), body: ls!("일·월·분기 단위 확인", "Daily, monthly, or quarterly checks") },
            Feature { title: ls!("기준 시점", "Reference Timestamp"), body: ls!("원장·지갑·온체인 대사 시점 불일치", "Mismatched reconciliation timestamps across ledger, wallet, and on-chain") },
            Feature { title: ls!("민감정보", "Sensitive Data"), body: ls!("고객 잔고·거래내역 직접 공개 곤란", "Difficulty directly disclosing customer balances and transaction history") },
            Feature { title: ls!("감사 대응", "Audit Response"), body: ls!("자료 분산·반복 취합 부담", "Burden of scattered and repeated data consolidation") },
        ],
        why_title: ls!("왜 영지식증명인가", "Why Zero-Knowledge Proofs?"),
        why_lead: ls!("투명성과 정보보호의 딜레마를 동시에 해결합니다.", "Resolving the dilemma between transparency and privacy at once."),
        answer_eyebrow: ls!("제품이 하는 일", "What the Product Does"),
        answer_title: ls!(
            "zkPoRL은 원장·지갑·온체인 상태를 하나의 확인 흐름으로 묶습니다.",
            "zkPoRL unifies ledger, wallet, and on-chain state into a single verification flow."
        ),
        answer_body: ls!(
            "원천 시스템을 대신하지 않습니다. 필요한 데이터를 읽고 정렬한 뒤, 정합성 확인 결과와 조치 기록을 남기도록 돕습니다.",
            "It does not replace source systems. It reads and aligns the necessary data, then helps record consistency results and action history."
        ),
        answer_cards: vec![
            Feature { title: ls!("PoL 상시 대사", "Continuous PoL Reconciliation"), body: ls!("고객부채 합계가 기준 데이터와 맞는지 짧은 주기로 확인합니다.", "Verifies on short cycles that total customer liabilities match reference data.") },
            Feature { title: ls!("민감정보 보호", "Sensitive Data Protection"), body: ls!("개별 잔고를 직접 보여주지 않고 필요한 확인 결과 중심으로 설명합니다.", "Explains results without directly exposing individual balances.") },
            Feature { title: ls!("이상징후 Case", "Anomaly Cases"), body: ls!("누락, 중복, 차이 후보를 운영자가 확인할 조치 항목으로 남깁니다.", "Records omission, duplication, and discrepancy candidates as action items for operators.") },
            Feature { title: ls!("증빙 패키지", "Evidence Package"), body: ls!("검증 실행 결과, 기준시점 정보, 조치 이력을 감사 참고자료로 묶습니다.", "Bundles verification results, reference timestamp info, and action history as audit reference materials.") },
        ],
        workflow_eyebrow: ls!("업무 흐름", "Workflow"),
        workflow_title: ls!(
            "데이터 확인부터 조치 기록까지 같은 화면 흐름으로 이어집니다.",
            "From data verification to action records — all in a single workflow."
        ),
        workflow_body: ls!(
            "운영자는 증명값 자체보다 오늘 확인해야 할 차이, 원인 후보, 조치 상태를 빠르게 보는 것이 중요합니다.",
            "Operators need to quickly see today's discrepancies, root cause candidates, and action status — not just the raw proof values."
        ),
        workflow: vec![
            ProcessItem { number: "01", title: ls!("데이터 읽기", "Read Data"), body: ls!("전산원장, 지갑 잔고, 온체인 상태를 읽습니다.", "Reads the accounting ledger, wallet balances, and on-chain state.") },
            ProcessItem { number: "02", title: ls!("기준 맞추기", "Align References"), body: ls!("자산 단위와 기준시점을 맞춰 비교 가능한 형태로 정리합니다.", "Aligns asset units and timestamps into a comparable format.") },
            ProcessItem { number: "03", title: ls!("PoL 확인", "PoL Verification"), body: ls!("고객부채 합계와 기준 데이터의 정합성을 확인합니다.", "Verifies consistency between total customer liabilities and reference data.") },
            ProcessItem { number: "04", title: ls!("이상징후 분류", "Anomaly Classification"), body: ls!("누락, 중복, 차이 후보를 조치할 항목으로 분리합니다.", "Separates omission, duplication, and discrepancy candidates into actionable items.") },
            ProcessItem { number: "05", title: ls!("증빙 정리", "Evidence Compilation"), body: ls!("검증 결과와 조치 이력을 감사 참고자료로 묶습니다.", "Bundles verification results and action history as audit reference materials.") },
        ],
        operations_eyebrow: ls!("데모", "Demo"),
        operations_title: ls!(
            "데모는 PoL 업무 화면을 중심으로 확인합니다.",
            "The demo focuses on the PoL operational screens."
        ),
        operations_body: ls!(
            "데모 화면은 상담 미팅에서 상시 대사와 이상징후 조치 흐름을 함께 확인하는 방식으로 운영합니다.",
            "The demo walkthrough covers continuous reconciliation and anomaly response flows during a consultation meeting."
        ),
        operations_demo_label: ls!("미팅 데모", "Meeting Demo"),
        operations: vec![
            Feature { title: ls!("ZP-1 상시 대사", "ZP-1 Continuous Reconciliation"), body: ls!("고객부채 합계를 개별 잔고 비공개 상태로 확인하고 대시보드에서 상태를 봅니다.", "Verifies total customer liability without exposing individual balances, with status visible on the dashboard.") },
            Feature { title: ls!("ZP-4 이상징후 조치", "ZP-4 Anomaly Response"), body: ls!("중복 유입이나 차이 후보를 확인하고 증명 실패, 지급 제한 검토 등 조치 기록으로 연결합니다.", "Reviews duplicate entries and discrepancy candidates, then links to action records such as proof failures or payment restrictions.") },
            Feature { title: ls!("운영자 대시보드", "Operator Dashboard"), body: ls!("검증 상태, 차이 후보, Case 진행 상태를 운영자가 한 화면에서 확인합니다.", "Operators see verification status, discrepancy candidates, and case progress on a single screen.") },
            Feature { title: ls!("PoR 확장", "PoR Extension"), body: ls!("7월 로드맵에서 준비자산 검증을 결합해 zkPoRL 흐름으로 넓힙니다.", "The July roadmap expands to include reserve asset verification, completing the zkPoRL flow.") },
        ],
        proof_eyebrow: ls!("확인 자료", "Evidence"),
        proof_title: ls!(
            "감사 의견을 대신하지 않고, 감사에 필요한 근거를 정리합니다.",
            "Not a substitute for audit opinions — it organizes the evidence audits need."
        ),
        proof_body: ls!(
            "zkPoRL은 규제 대응을 보장하는 문구보다, 운영 중 확인한 사실과 조치 기록을 일관되게 남기는 데 초점을 둡니다.",
            "zkPoRL focuses on consistently recording verified facts and action logs during operations, rather than making regulatory compliance guarantees."
        ),
        proof_points: vec![
            Feature { title: ls!("검증 실행 요약", "Verification Run Summary"), body: ls!("어느 기간, 어떤 데이터 기준으로 확인했는지 남깁니다.", "Records what period and data basis was used for verification.") },
            Feature { title: ls!("기준시점 기록", "Reference Timestamp Record"), body: ls!("비교에 사용한 데이터 시점과 무결성 근거를 남깁니다.", "Records the data timestamp and integrity basis used in comparison.") },
            Feature { title: ls!("Alert / Case", "Alert / Case"), body: ls!("이상징후 후보와 담당자 조치 이력을 연결합니다.", "Links anomaly candidates with the responsible party's action history.") },
            Feature { title: ls!("공시·감사 참고자료", "Disclosure & Audit Reference"), body: ls!("외부 설명에 필요한 요약 자료를 반복 생성할 수 있게 돕습니다.", "Helps repeatedly generate summary materials needed for external explanations.") },
        ],
        cta_eyebrow: ls!("다음 단계", "Next Steps"),
        cta_title: ls!(
            "PoL 상시 대사부터 PoR 결합 로드맵까지 함께 검토하세요.",
            "Review the roadmap from continuous PoL reconciliation to full PoR integration."
        ),
        cta_body: ls!(
            "거래소, 수탁사, 은행의 현재 데이터 구조를 기준으로 PoL 적용 범위와 PoR 확장 시점을 나눠 설계할 수 있습니다.",
            "Based on your current data structure — whether exchange, custodian, or bank — we can design the PoL scope and PoR expansion timeline together."
        ),
        related_label: ls!("함께 검토할 제품", "Related Product"),
        related_name: "zkWallet",
        related_body: ls!(
            "기관용 지갑 생성, 수탁 입출금, 발행·소각 승인 흐름도 함께 검토할 수 있습니다.",
            "Institutional wallet creation, custody deposits/withdrawals, and issuance/burn approval flows can be reviewed together."
        ),
        related_href: "/products/zkwallet",
        related_link_label: ls!("zkWallet 보기", "View zkWallet"),
        // zkPoRL specific
        flow_section_title: ls!("zkPoRL Server 처리 흐름", "zkPoRL Server Processing Flow"),
        flow_subtitle: ls!(
            "장부·지갑·온체인·감사증빙을 하나의 검증 흐름으로 연결합니다.",
            "Linking ledger, wallet, on-chain data, and audit evidence in a single verification flow."
        ),
        flow_in1_title: ls!("전산원장", "Accounting Ledger"),
        flow_in2_title: ls!("지갑 잔고", "Wallet Balance"),
        flow_in3_title: ls!("온체인 데이터", "On-chain Data"),
        flow_out1_title: ls!("운영 대시보드", "Operations Dashboard"),
        flow_out1_sub: ls!("상태·diff·alert", "status · diff · alert"),
        flow_out2_title: ls!("Case 관리", "Case Management"),
        flow_out2_sub: ls!("증명 실패 원인 확인", "Identify proof failure causes"),
        flow_out3_title: ls!("감사·공시 자료", "Audit & Disclosure Evidence"),
        flow_server_desc: ls!(
            "정의된 입력과 검증 명제에 대해\n계산 정합성을 확인합니다.",
            "Verifies computational consistency\nof defined inputs and propositions."
        ),
        flow_note: ls!(
            "zkPoRL은 원천 데이터를 생성하지 않습니다. 정의된 데이터와 검증 명제에 대한 정합성을 확인합니다.",
            "zkPoRL does not generate source data. It verifies the consistency of defined data against verification propositions."
        ),
        table_header1: ls!("구분", "Category"),
        table_header2: ls!("일반 대사", "Traditional"),
        table_row1_label: ls!("검증방식", "Verification"),
        table_row1_traditional: ls!("원본 직접 대조", "Direct comparison"),
        table_row1_new: ls!("증명값 기반 검증", "Proof-based verification"),
        table_row2_label: ls!("정보 노출", "Data Exposure"),
        table_row2_traditional: ls!("민감정보 노출 위험", "Risk of exposure"),
        table_row2_new: ls!("민감정보 비공개", "Data kept private"),
        table_row3_label: ls!("공시 신뢰", "Disclosure Trust"),
        table_row3_traditional: ls!("운영기관 발표 의존", "Relies on operator"),
        table_row3_new: ls!("수학적 검증 가능", "Mathematically verifiable"),
        // zkWallet specific (unused in zkPoRL)
        mpc_asis_title: ls!("", ""),
        mpc_asis_item1: ls!("", ""),
        mpc_asis_item2: ls!("", ""),
        mpc_asis_item3: ls!("", ""),
        mpc_tobe_title: ls!("", ""),
        mpc_tobe_item1: ls!("", ""),
        mpc_tobe_item2: ls!("", ""),
        mpc_tobe_item3: ls!("", ""),
        mpc_section_title: ls!("", ""),
        mpc_section_lead: ls!("", ""),
        mpc_shard_label: ls!("", ""),
        mpc_partial_sig: ls!("", ""),
        mpc_collect_label: ls!("", ""),
        mpc_chain_label: ls!("", ""),
        mpc_note: ls!("", ""),
    }
}

fn zkwallet_landing() -> ProductLanding {
    ProductLanding {
        theme_class: "product-theme-wallet",
        title: ls!(
            "zkWallet Custody | MPC 기반 고보안 디지털자산 지갑",
            "zkWallet Custody | High-Security MPC-Based Digital Asset Wallet"
        ),
        description: ls!(
            "zkWallet Custody는 개인키를 단일 지점에 보관하지 않는 MPC 기반 키 관리·서명 Wallet Service입니다.",
            "zkWallet Custody is an MPC-based key management and signing Wallet Service that never stores private keys in a single location."
        ),
        product_name: ls!(
            "zkWallet Custody : MPC 기반 고보안 Wallet Service",
            "zkWallet Custody : High-Security MPC-Based Wallet Service"
        ),
        eyebrow: ls!("zkWallet Custody", "zkWallet Custody"),
        hero_title: ls!(
            "키를 한 곳에 모으지 않는 기관형 디지털자산 지갑",
            "Institutional Digital Asset Wallet — Keys Never Concentrated in One Place"
        ),
        hero_lead: ls!(
            "zkWallet Custody는 개인키를 여러 조각으로 나눠 보관하고, 정책 검증을 거친 거래만 여러 노드가 공동 서명하는 MPC 기반 고보안 Wallet Service입니다.",
            "zkWallet Custody splits private keys into multiple shards and only co-signs policy-verified transactions across multiple nodes — an MPC-based high-security Wallet Service."
        ),
        hero_note: ls!(
            "키 보관을 넘어 생성·갱신·복구·폐기와 기관형 통제까지 제공합니다.",
            "Beyond key storage — providing generation, rotation, recovery, revocation, and institutional-grade control."
        ),
        primary_cta: Cta { label: ls!("Contact", "Contact"), href: "/#contact" },
        secondary_cta: Cta { label: ls!("제품군으로 돌아가기", "Back to Products"), href: "/#products" },
        stats: vec![
            Metric {
                value: ls!("분산 키 서명", "Distributed Signing"),
                label: ls!("MPC", "MPC"),
                detail: ls!("키 원본 없이 (t/n) 공동 서명", "Threshold (t/n) co-signing without a full key"),
            },
            Metric {
                value: ls!("전 주기 관리", "Full Lifecycle"),
                label: ls!("키 생애주기", "Key Lifecycle"),
                detail: ls!("생성·갱신·복구·폐기", "Generation, rotation, recovery, revocation"),
            },
            Metric {
                value: ls!("정책·감사", "Policy & Audit"),
                label: ls!("기관형 통제", "Institutional Control"),
                detail: ls!("승인·추적·고가용성", "Approval, traceability, high availability"),
            },
        ],
        problem_title: ls!(
            "단일 마스터 시드 구조는 키 하나가 뚫리면 전체 자산이 위험합니다.",
            "A single master seed means one key compromise risks all assets."
        ),
        problem_body: ls!(
            "디지털자산 지갑에서 개인키는 곧 자산 접근 권한입니다. 하나의 니모닉·마스터 시드가 유출되면 전체 자산 탈취로 이어질 수 있습니다.",
            "In a digital asset wallet, the private key is asset access itself. A single leaked mnemonic or master seed can lead to total asset theft."
        ),
        problem_callout_badge: ls!("", ""),
        problem_callout_body: ls!("", ""),
        problem_summary: ls!(
            "문제는 키를 어떻게 보관하느냐가 아니라, 키를 한 곳에 모으지 않는 것입니다.",
            "The question is not how to store the key — it is ensuring the key is never concentrated in one place."
        ),
        problems: vec![
            Feature { title: ls!("단일 키 유출", "Single Key Exposure"), body: ls!("마스터 시드 하나의 유출이 전체 자산 탈취로 이어집니다.", "A single master seed leak leads to complete asset theft.") },
            Feature { title: ls!("권한 오남용", "Authorization Abuse"), body: ls!("발행·출금 권한이 집중되면 무단 실행 위험이 커집니다.", "Concentrated issuance and withdrawal authority increases the risk of unauthorized execution.") },
            Feature { title: ls!("통제 부재", "Lack of Control"), body: ls!("고위험 작업이 승인·정책 검증 없이 실행될 수 있습니다.", "High-risk operations can execute without approval or policy verification.") },
            Feature { title: ls!("추적·복구 곤란", "Difficult Tracing & Recovery"), body: ls!("문제 원인 파악과 장애·키 복구가 어렵습니다.", "Identifying root causes and recovering from failures or key issues is difficult.") },
        ],
        why_title: ls!("왜 MPC인가", "Why MPC?"),
        why_lead: ls!("개인키를 한 곳에 모으지 않아 단일 키 유출 위험을 없앱니다.", "Eliminating single-key exposure risk by never concentrating the private key in one place."),
        answer_eyebrow: ls!("제품이 하는 일", "What the Product Does"),
        answer_title: ls!(
            "금융 서비스와 블록체인 사이에서 키 관리·서명을 안전하게 처리합니다.",
            "Securely handles key management and signing between financial services and the blockchain."
        ),
        answer_body: ls!(
            "금융 서비스의 키 생성·거래 서명 요청을 받아 MPC로 처리하고, 서명된 거래를 블록체인에 전송합니다.",
            "Receives key generation and transaction signing requests from financial services, processes them via MPC, and broadcasts signed transactions to the blockchain."
        ),
        answer_cards: vec![
            Feature { title: ls!("키 관리", "Key Management"), body: ls!("키를 생성하고 잠금·해제를 관리합니다.", "Generates keys and manages locking and unlocking.") },
            Feature { title: ls!("지갑 관리", "Wallet Management"), body: ls!("지갑을 생성하고 주소를 발급합니다.", "Creates wallets and issues addresses.") },
            Feature { title: ls!("서명 관리", "Signature Management"), body: ls!("서명을 생성하고 검증합니다.", "Generates and verifies signatures.") },
            Feature { title: ls!("정책 관리", "Policy Management"), body: ls!("거래 한도와 출금 승인 정책을 적용합니다.", "Applies transaction limits and withdrawal approval policies.") },
            Feature { title: ls!("MPC 운영 관리", "MPC Operations"), body: ls!("서명 세션과 참여 노드를 관리합니다.", "Manages signing sessions and participating nodes.") },
            Feature { title: ls!("보안 저장소 관리", "Secure Storage Management"), body: ls!("Secret과 Master Key를 안전하게 보관합니다.", "Securely stores secrets and master keys.") },
        ],
        workflow_eyebrow: ls!("업무 흐름", "Workflow"),
        workflow_title: ls!(
            "정책 검증을 거친 거래만 여러 노드가 공동 서명합니다.",
            "Only policy-verified transactions are co-signed across multiple nodes."
        ),
        workflow_body: ls!(
            "키 조각 A·B·C가 부분 서명에 참여하고, Manager가 이를 결합해 최종 서명을 생성합니다.",
            "Key shards A, B, and C participate in partial signing, and the Manager combines them to produce the final signature."
        ),
        workflow: vec![
            ProcessItem { number: "01", title: ls!("거래 서명 요청", "Transaction Signing Request"), body: ls!("금융 서비스로부터 거래 서명 요청을 받습니다.", "Receives a transaction signing request from the financial service.") },
            ProcessItem { number: "02", title: ls!("정책 검증·승인", "Policy Verification & Approval"), body: ls!("정책을 검증한 뒤 서명 절차로 진행합니다.", "Verifies policy compliance before proceeding to the signing process.") },
            ProcessItem { number: "03", title: ls!("MPC 공동 서명", "MPC Co-Signing"), body: ls!("Party Node A·B·C가 부분 서명을 생성합니다.", "Party Nodes A, B, and C each generate a partial signature.") },
            ProcessItem { number: "04", title: ls!("서명 결합", "Signature Aggregation"), body: ls!("Manager가 부분 서명을 모아 서명을 생성합니다.", "The Manager aggregates the partial signatures into a final signature.") },
            ProcessItem { number: "05", title: ls!("결과 반환", "Result Return"), body: ls!("생성된 서명 결과를 금융 서비스로 반환합니다.", "Returns the generated signature result to the financial service.") },
        ],
        operations_eyebrow: ls!("데모", "Demo"),
        operations_title: ls!(
            "고객군별 시나리오로 도입 흐름을 확인합니다.",
            "Explore deployment flows through customer-specific scenarios."
        ),
        operations_body: ls!(
            "개인 사용자, 수탁사, 발행사, 금융사 시나리오로 지갑 개설부터 거래·발행 흐름을 확인합니다.",
            "Walk through wallet creation to transaction and issuance flows for individual users, custodians, issuers, and financial institutions."
        ),
        operations_demo_label: ls!("시나리오", "Scenario"),
        operations: vec![
            Feature { title: ls!("개인 사용자", "Individual User"), body: ls!("앱에서 지갑을 개설하고, 전송 시 MPC 서명으로 거래합니다.", "Opens a wallet in the app and transacts using MPC signing for transfers.") },
            Feature { title: ls!("수탁사", "Custodian"), body: ls!("수탁 지갑을 개설하고 다자 승인 기반으로 입·출금을 처리합니다.", "Opens a custody wallet and processes deposits and withdrawals with multi-party approval.") },
            Feature { title: ls!("스테이블코인 발행사", "Stablecoin Issuer"), body: ls!("준비금 확인과 승인 절차를 거쳐 Mint/Burn을 실행합니다.", "Executes Mint/Burn after reserve verification and approval procedures.") },
            Feature { title: ls!("금융사", "Financial Institution"), body: ls!("슈퍼앱에 Wallet Service를 연계해 디지털자산 서비스를 확장합니다.", "Connects Wallet Service to a super-app to expand digital asset services.") },
        ],
        proof_eyebrow: ls!("확인 자료", "Evidence"),
        proof_title: ls!(
            "서명 기능을 넘어, 키 생애주기·실행환경·통제를 결합합니다.",
            "Beyond signing — combining key lifecycle management, execution environment, and institutional control."
        ),
        proof_body: ls!(
            "기관형 Wallet Service는 서명만으로 충분하지 않습니다. zkWallet Custody는 키 생애주기 관리, 보호 실행 환경, 기관형 통제를 함께 제공합니다.",
            "An institutional Wallet Service needs more than just signing. zkWallet Custody provides key lifecycle management, protected execution environment, and institutional controls together."
        ),
        proof_points: vec![
            Feature { title: ls!("키 생애주기 관리", "Key Lifecycle Management"), body: ls!("생성(DKG)·갱신(Proactive Refresh)·복구·폐기를 통합 관리합니다.", "Unified management of generation (DKG), rotation (Proactive Refresh), recovery, and revocation.") },
            Feature { title: ls!("보호 실행 환경", "Protected Execution Environment"), body: ls!("Confidential Computing과 외부 KMS/HSM 연동으로 키 연산을 보호합니다.", "Protects key operations via Confidential Computing and external KMS/HSM integration.") },
            Feature { title: ls!("MPC 분산 서명", "MPC Distributed Signing"), body: ls!("키 조각이 한곳에 모이지 않는 (t/n) 임계치 공동 서명을 수행합니다.", "Performs (t/n) threshold co-signing without ever concentrating key shards in one place.") },
            Feature { title: ls!("기관형 통제", "Institutional Control"), body: ls!("Policy Engine, Identifiable Abort, 고가용성 구조로 운영을 통제합니다.", "Controls operations via Policy Engine, Identifiable Abort, and high-availability architecture.") },
        ],
        cta_eyebrow: ls!("다음 단계", "Next Steps"),
        cta_title: ls!(
            "도입 환경에 맞는 Wallet Service 제공 방식을 함께 설계하세요.",
            "Design the right Wallet Service deployment model for your environment."
        ),
        cta_body: ls!(
            "설치형(서버·클라우드)과 Confidential Computing 서버 일체형 중 보안 요구수준에 맞는 방식을 검토합니다.",
            "Review whether an on-premise (server/cloud) or Confidential Computing all-in-one deployment best fits your security requirements."
        ),
        related_label: ls!("함께 검토할 제품", "Related Product"),
        related_name: "zkPoRL",
        related_body: ls!(
            "고객부채 상시 대사와 준비자산 검증 로드맵도 함께 검토할 수 있습니다.",
            "Continuous customer liability reconciliation and the reserve asset verification roadmap can be reviewed together."
        ),
        related_href: "/products/zkporl",
        related_link_label: ls!("zkPoRL 보기", "View zkPoRL"),
        // zkWallet specific
        mpc_asis_title: ls!("단일 마스터 시드", "Single Master Seed"),
        mpc_asis_item1: ls!("키를 한 곳에 집중 보관합니다.", "Keys are stored in one concentrated location."),
        mpc_asis_item2: ls!("단일 유출 시 전체 자산이 탈취됩니다.", "A single breach leads to total asset theft."),
        mpc_asis_item3: ls!("키 원본이 복원될 수 있습니다.", "The original key can be reconstructed."),
        mpc_tobe_title: ls!("MPC 분산 키", "MPC Distributed Keys"),
        mpc_tobe_item1: ls!("개인키를 여러 조각으로 나눠 분산 보관합니다.", "Private keys are split into shards and stored separately."),
        mpc_tobe_item2: ls!("(t/n) 임계치 기반으로 공동 서명합니다.", "Co-signing is based on a (t/n) threshold."),
        mpc_tobe_item3: ls!("어떤 경우에도 키 원본이 복원되지 않습니다.", "The original key is never reconstructed under any circumstance."),
        mpc_section_title: ls!("정책을 통과한 거래만 공동 서명합니다", "Only policy-verified transactions are co-signed."),
        mpc_section_lead: ls!(
            "키 조각 A·B·C가 각각 부분 서명을 만들고, Manager가 이를 결합해 최종 서명을 생성합니다.",
            "Key shards A, B, and C each produce a partial signature, which the Manager combines into a final signature."
        ),
        mpc_shard_label: ls!("키 조각", "Key Shard"),
        mpc_partial_sig: ls!("부분 서명", "Partial Signature"),
        mpc_collect_label: ls!("부분 서명 수집 · 결합", "Collect & Combine Partial Signatures"),
        mpc_chain_label: ls!("블록체인 전송", "Broadcast to Blockchain"),
        mpc_note: ls!(
            "거래 서명 요청은 정책 검증 후에만 진행되며, 키 원본을 모으지 않고 부분 서명의 결합으로 완성됩니다.",
            "Transaction signing requests proceed only after policy verification, and are completed by combining partial signatures — never by reconstructing the original key."
        ),
        // zkPoRL specific (unused in zkWallet)
        flow_section_title: ls!("", ""),
        flow_subtitle: ls!("", ""),
        flow_in1_title: ls!("", ""),
        flow_in2_title: ls!("", ""),
        flow_in3_title: ls!("", ""),
        flow_out1_title: ls!("", ""),
        flow_out1_sub: ls!("", ""),
        flow_out2_title: ls!("", ""),
        flow_out2_sub: ls!("", ""),
        flow_out3_title: ls!("", ""),
        flow_server_desc: ls!("", ""),
        flow_note: ls!("", ""),
        table_header1: ls!("", ""),
        table_header2: ls!("", ""),
        table_row1_label: ls!("", ""),
        table_row1_traditional: ls!("", ""),
        table_row1_new: ls!("", ""),
        table_row2_label: ls!("", ""),
        table_row2_traditional: ls!("", ""),
        table_row2_new: ls!("", ""),
        table_row3_label: ls!("", ""),
        table_row3_traditional: ls!("", ""),
        table_row3_new: ls!("", ""),
    }
}

fn zkvoting_landing() -> ProductLanding {
    ProductLanding {
        theme_class: "product-theme-voting",
        title: ls!(
            "zkVoting | 영지식증명 기반 비밀투표·검증 시스템",
            "zkVoting | ZKP-Based Secret Ballot & Verification System"
        ),
        description: ls!(
            "zkVoting은 개별 표를 공개하지 않고도 집계 결과를 누구나 검증할 수 있는 ZKP 기반 전자투표 솔루션입니다.",
            "zkVoting is a ZKP-based electronic voting solution that lets anyone verify the tally — without revealing individual ballots."
        ),
        product_name: ls!("zkVoting : Verifiable Private Voting", "zkVoting : Verifiable Private Voting"),
        eyebrow: ls!("zkVoting", "zkVoting"),
        hero_title: ls!(
            "비밀은 지키고, 결과는 증명하는 투표",
            "Voting that keeps secrets and proves results."
        ),
        hero_lead: ls!(
            "zkVoting은 개별 표를 공개하지 않고도 집계 결과의 정합성을 누구나 검증할 수 있게 합니다.",
            "zkVoting lets anyone verify the consistency of tally results — without exposing individual ballots."
        ),
        hero_note: ls!(
            "투표 비밀성과 결과 검증가능성을 동시에 보장합니다.",
            "Guarantees both ballot secrecy and result verifiability simultaneously."
        ),
        primary_cta: Cta { label: ls!("이동하기", "Visit Site"), href: "https://www.zkvoting.com/" },
        secondary_cta: Cta { label: ls!("제품군으로 돌아가기", "Back to Products"), href: "/#products" },
        stats: vec![
            Metric { value: ls!("비밀투표", "Secret Ballot"), label: ls!("프라이버시", "Privacy"), detail: ls!("개별 표 비공개", "Individual ballots kept private") },
            Metric { value: ls!("공개 검증", "Public Verification"), label: ls!("무결성", "Integrity"), detail: ls!("집계 결과 검증 가능", "Tally results verifiable by anyone") },
            Metric { value: ls!("1인 1표", "One Person, One Vote"), label: ls!("자격 검증", "Eligibility"), detail: ls!("이중·부정 투표 차단", "Double and invalid votes blocked") },
        ],
        problem_title: ls!(
            "전자투표는 비밀성과 검증가능성을 동시에 충족하기 어렵습니다.",
            "Electronic voting struggles to satisfy both secrecy and verifiability."
        ),
        problem_body: ls!(
            "투명하게 공개하면 비밀투표가 깨지고, 비공개로 운영하면 결과를 신뢰하기 어렵습니다.",
            "Full transparency breaks ballot secrecy; full privacy makes results hard to trust."
        ),
        problem_callout_badge: ls!("", ""),
        problem_callout_body: ls!("", ""),
        problem_summary: ls!("", ""),
        problems: vec![
            Feature { title: ls!("비밀 보장", "Ballot Secrecy"), body: ls!("누가 무엇에 투표했는지 드러나면 안 됩니다.", "Who voted for what must never be revealed.") },
            Feature { title: ls!("결과 검증", "Result Verification"), body: ls!("집계가 조작되지 않았음을 외부가 확인해야 합니다.", "External parties must be able to confirm the tally was not manipulated.") },
            Feature { title: ls!("자격 확인", "Eligibility Check"), body: ls!("유권자 자격과 1인 1표를 보장해야 합니다.", "Voter eligibility and the one-person-one-vote rule must be enforced.") },
            Feature { title: ls!("운영 신뢰", "Operational Trust"), body: ls!("운영기관 발표에만 의존하기 어렵습니다.", "Relying solely on the operator's announcement is insufficient.") },
        ],
        why_title: ls!("", ""),
        why_lead: ls!("", ""),
        answer_eyebrow: ls!("제품이 하는 일", "What the Product Does"),
        answer_title: ls!(
            "zkVoting은 표를 공개하지 않고 결과만 증명합니다.",
            "zkVoting proves only the result — never the individual ballot."
        ),
        answer_body: ls!(
            "유권자 자격 검증, 암호화 투표, 영지식 집계 증명, 공개 검증을 하나의 흐름으로 제공합니다.",
            "Provides voter eligibility verification, encrypted voting, ZK tally proof, and public verification in a single flow."
        ),
        answer_cards: vec![
            Feature { title: ls!("유권자 등록", "Voter Registration"), body: ls!("자격을 검증하고 익명 자격 증명을 발급합니다.", "Verifies eligibility and issues anonymous credentials.") },
            Feature { title: ls!("비밀 투표", "Secret Ballot"), body: ls!("표를 암호화해 내용을 비공개로 제출합니다.", "Encrypts and submits ballots while keeping their content private.") },
            Feature { title: ls!("영지식 집계", "ZK Tally"), body: ls!("복호화 없이 집계 정합성을 증명합니다.", "Proves tally consistency without decryption.") },
            Feature { title: ls!("공개 검증", "Public Verification"), body: ls!("누구나 결과 증명을 검증할 수 있습니다.", "Anyone can verify the result proof.") },
        ],
        workflow_eyebrow: ls!("업무 흐름", "Workflow"),
        workflow_title: ls!(
            "등록부터 검증까지 하나의 검증 가능한 흐름",
            "A single verifiable flow from registration to verification."
        ),
        workflow_body: ls!(
            "각 단계가 암호학적 증거로 연결되어 사후 검증이 가능합니다.",
            "Each step is linked by cryptographic evidence, enabling post-hoc verification."
        ),
        workflow: vec![
            ProcessItem { number: "01", title: ls!("유권자 등록", "Voter Registration"), body: ls!("자격을 확인하고 익명 자격 증명을 발급합니다.", "Verifies eligibility and issues anonymous credentials.") },
            ProcessItem { number: "02", title: ls!("비밀 투표", "Secret Ballot"), body: ls!("내용을 암호화한 표를 제출합니다.", "Submits an encrypted ballot.") },
            ProcessItem { number: "03", title: ls!("무결성 검증", "Integrity Verification"), body: ls!("중복·부정 투표 후보를 차단합니다.", "Blocks duplicate and invalid vote candidates.") },
            ProcessItem { number: "04", title: ls!("영지식 집계", "ZK Tally"), body: ls!("표를 공개하지 않고 결과를 계산·증명합니다.", "Computes and proves the result without revealing ballots.") },
            ProcessItem { number: "05", title: ls!("결과 공개·검증", "Result Publication & Verification"), body: ls!("결과와 증명을 공개해 검증할 수 있게 합니다.", "Publishes results and proofs for public verification.") },
        ],
        operations_eyebrow: ls!("데모 확인 방식", "Demo Format"),
        operations_title: ls!(
            "데모는 투표 운영 화면을 중심으로 확인합니다.",
            "The demo focuses on the voting administration screens."
        ),
        operations_demo_label: ls!("미팅 데모", "Meeting Demo"),
        operations_body: ls!(
            "데모는 상담 미팅에서 투표 생성부터 결과 검증까지 함께 확인하는 방식으로 운영합니다.",
            "The demo walkthrough covers the full flow from ballot creation to result verification during a consultation meeting."
        ),
        operations: vec![
            Feature { title: ls!("투표 생성", "Ballot Creation"), body: ls!("후보·문항과 유권자 명부를 설정합니다.", "Sets up candidates, questions, and the voter registry.") },
            Feature { title: ls!("투표 진행", "Voting Session"), body: ls!("유권자 인증과 암호화 투표를 운영합니다.", "Manages voter authentication and encrypted ballot submission.") },
            Feature { title: ls!("집계·검증", "Tally & Verification"), body: ls!("영지식 집계 결과와 증명을 확인합니다.", "Reviews the ZK tally result and its proof.") },
            Feature { title: ls!("감사·공개", "Audit & Publication"), body: ls!("검증용 자료를 외부에 공개합니다.", "Publishes verification materials for external review.") },
        ],
        proof_eyebrow: ls!("확인 자료", "Evidence"),
        proof_title: ls!(
            "결과 발표를 대신하지 않고, 누구나 검증할 근거를 제공합니다.",
            "Not a result announcement — it provides evidence anyone can verify."
        ),
        proof_body: ls!(
            "zkVoting은 신뢰 선언보다, 암호학적으로 검증 가능한 증거를 일관되게 남기는 데 초점을 둡니다.",
            "zkVoting focuses on consistently producing cryptographically verifiable evidence, rather than making trust declarations."
        ),
        proof_points: vec![
            Feature { title: ls!("유권자 자격 증명", "Voter Credential"), body: ls!("자격과 1인 1표를 익명으로 보장합니다.", "Guarantees eligibility and one-person-one-vote anonymously.") },
            Feature { title: ls!("투표 비밀성", "Ballot Secrecy"), body: ls!("개별 표의 내용을 공개하지 않습니다.", "Individual ballot contents are never revealed.") },
            Feature { title: ls!("집계 정합성 증명", "Tally Consistency Proof"), body: ls!("결과가 제출된 표와 일치함을 증명합니다.", "Proves that the result matches the submitted ballots.") },
            Feature { title: ls!("공개 검증 자료", "Public Verification Materials"), body: ls!("누구나 결과를 검증할 수 있게 공개합니다.", "Published so anyone can verify the results.") },
        ],
        cta_eyebrow: ls!("다음 단계", "Next Steps"),
        cta_title: ls!(
            "검증 가능한 비밀투표를 직접 확인해보세요.",
            "See verifiable secret voting in action."
        ),
        cta_body: ls!(
            "선거, 총회, 거버넌스 등 적용 시나리오에 맞춰 PoC 범위를 함께 설계할 수 있습니다.",
            "We can design a PoC scope together tailored to your scenario — elections, general meetings, governance, and more."
        ),
        related_label: ls!("함께 검토할 제품", "Related Product"),
        related_name: "zkPoRL",
        related_body: ls!(
            "고객부채 상시 대사와 준비자산 검증 로드맵도 함께 검토할 수 있습니다.",
            "Continuous customer liability reconciliation and the reserve asset verification roadmap can be reviewed together."
        ),
        related_href: "/products/zkporl",
        related_link_label: ls!("zkPoRL 보기", "View zkPoRL"),
        // zkPoRL / zkWallet specific (unused in zkVoting)
        mpc_asis_title: ls!("", ""),
        mpc_asis_item1: ls!("", ""),
        mpc_asis_item2: ls!("", ""),
        mpc_asis_item3: ls!("", ""),
        mpc_tobe_title: ls!("", ""),
        mpc_tobe_item1: ls!("", ""),
        mpc_tobe_item2: ls!("", ""),
        mpc_tobe_item3: ls!("", ""),
        mpc_section_title: ls!("", ""),
        mpc_section_lead: ls!("", ""),
        mpc_shard_label: ls!("", ""),
        mpc_partial_sig: ls!("", ""),
        mpc_collect_label: ls!("", ""),
        mpc_chain_label: ls!("", ""),
        mpc_note: ls!("", ""),
        flow_section_title: ls!("", ""),
        flow_subtitle: ls!("", ""),
        flow_in1_title: ls!("", ""),
        flow_in2_title: ls!("", ""),
        flow_in3_title: ls!("", ""),
        flow_out1_title: ls!("", ""),
        flow_out1_sub: ls!("", ""),
        flow_out2_title: ls!("", ""),
        flow_out2_sub: ls!("", ""),
        flow_out3_title: ls!("", ""),
        flow_server_desc: ls!("", ""),
        flow_note: ls!("", ""),
        table_header1: ls!("", ""),
        table_header2: ls!("", ""),
        table_row1_label: ls!("", ""),
        table_row1_traditional: ls!("", ""),
        table_row1_new: ls!("", ""),
        table_row2_label: ls!("", ""),
        table_row2_traditional: ls!("", ""),
        table_row2_new: ls!("", ""),
        table_row3_label: ls!("", ""),
        table_row3_traditional: ls!("", ""),
        table_row3_new: ls!("", ""),
    }
}
