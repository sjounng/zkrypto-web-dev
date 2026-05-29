#[derive(Clone)]
pub struct SiteContent {
    pub title: &'static str,
    pub description: &'static str,
    pub logo_src: &'static str,
    pub logo_alt: &'static str,
    pub favicon_src: &'static str,
    pub hero: Hero,
    pub ecosystem: Ecosystem,
    pub standard: Standard,
    pub use_cases: Vec<UseCase>,
    pub product_gateway: ProductGateway,
    pub trust_proof: TrustProof,
    pub final_cta: FinalCta,
    pub footer_columns: Vec<FooterColumn>,
    pub footer_text: &'static str,
    pub footer_note: &'static str,
}

#[derive(Clone)]
pub struct Hero {
    pub title: &'static str,
    pub sublead: &'static str,
    pub lead: &'static str,
    pub primary_cta: Cta,
    pub secondary_cta: Cta,
    pub proof_label: &'static str,
}

#[derive(Clone)]
pub struct Cta {
    pub label: &'static str,
    pub href: &'static str,
}

#[derive(Clone)]
pub struct Standard {
    pub eyebrow: &'static str,
    pub title: &'static str,
    pub body: &'static str,
    pub source_label: &'static str,
    pub source_standard: &'static str,
    pub source_main_title: &'static str,
    pub source_subtitle_prefix: &'static str,
    pub source_subtitle_highlight: &'static str,
    pub source_url: &'static str,
    pub source_link_label: &'static str,
    pub image_src: &'static str,
    pub image_alt: &'static str,
}

#[derive(Clone)]
pub struct UseCase {
    pub title: &'static str,
    pub detail: &'static str,
    pub class_name: &'static str,
    pub tone_class: &'static str,
    pub icon: &'static str,
}

#[derive(Clone)]
pub struct ProductGateway {
    pub eyebrow: &'static str,
    pub title: &'static str,
    pub cards: Vec<ProductCard>,
}

#[derive(Clone)]
pub struct ProductCard {
    pub name: &'static str,
    pub href: &'static str,
    pub cta_label: &'static str,
    pub tone_class: &'static str,
    pub image_src: &'static str,
    pub image_alt: &'static str,
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
    pub lead: &'static str,
    pub milestones: Vec<ProofMilestone>,
    pub proof_items: Vec<ProofItem>,
}

#[derive(Clone)]
pub struct ProofMilestone {
    pub phase: &'static str,
    pub title: &'static str,
    pub body: &'static str,
}

#[derive(Clone)]
pub struct ProofItem {
    pub label: &'static str,
    pub title: &'static str,
    pub detail: &'static str,
    pub tone_class: &'static str,
}

#[derive(Clone)]
pub struct FinalCta {
    pub eyebrow: &'static str,
    pub title: &'static str,
    pub body: &'static str,
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
    pub title: &'static str,
    pub description: &'static str,
    pub product_name: &'static str,
    pub eyebrow: &'static str,
    pub hero_title: &'static str,
    pub hero_lead: &'static str,
    pub hero_note: &'static str,
    pub primary_cta: Cta,
    pub secondary_cta: Cta,
    pub stats: Vec<Metric>,
    pub problem_title: &'static str,
    pub problem_body: &'static str,
    pub problems: Vec<Feature>,
    pub answer_title: &'static str,
    pub answer_body: &'static str,
    pub answer_cards: Vec<Feature>,
    pub workflow_title: &'static str,
    pub workflow_body: &'static str,
    pub workflow: Vec<ProcessItem>,
    pub operations_title: &'static str,
    pub operations_body: &'static str,
    pub operations: Vec<Feature>,
    pub proof_title: &'static str,
    pub proof_body: &'static str,
    pub proof_points: Vec<Feature>,
    pub cta_title: &'static str,
    pub cta_body: &'static str,
}

#[derive(Clone)]
pub struct Metric {
    pub value: &'static str,
    pub label: &'static str,
    pub detail: &'static str,
}

#[derive(Clone)]
pub struct Feature {
    pub title: &'static str,
    pub body: &'static str,
}

#[derive(Clone)]
pub struct ProcessItem {
    pub number: &'static str,
    pub title: &'static str,
    pub body: &'static str,
}

pub fn site_content() -> SiteContent {
    SiteContent {
        title: "ZKRYPTO | ISO/IEC 27565:2026 표준 기반 ZKP 신뢰 레이어",
        description: "지크립토는 ISO/IEC 27565:2026 글로벌 표준 기반 ZKP 기술로 데이터 공개를 최소화하고 필요한 사실만 증명하는 프라이버시 검증 구조를 설계합니다.",
        logo_src: "/assets/zkrypto_logo.png",
        logo_alt: "ZKRYPTO",
        favicon_src: "/assets/zkrypto-favicon.png",
        hero: Hero {
            title: "프라이버시와 컴플라이언스를\nZKP로 연결합니다.",
            sublead: "ZKP(Zero-knowledge Proofs)는 ISO/IEC에서도 표준화한 암호학적 체계입니다. ",
            lead: "지크립토는 ZKP를 활용하여 정보를 직접 공개하지 않고도, 필요한 사실만 증명할 수 있는 신뢰 레이어를 제공합니다.",
            primary_cta: Cta { label: "ZKP Product", href: "#products" },
            secondary_cta: Cta { label: "Contact", href: "#contact" },
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
        standard: Standard {
            eyebrow: "Global Standard",
            title: "지크립토는 ISO/IEC 27565:2026과 함께 글로벌을 선도합니다.",
            body: "생년월일을 몰라도 성인 여부를 확인 가능하고, 거래내역 전체를 몰라도 거래 유효성을 검증할 수 있습니다.\nISO/IEC 27565:2026은 이러한 방식의 프라이버시 강화 기술을 다루는 글로벌 표준입니다.",
            source_label: "",
            source_standard: "ISO/IEC 27565:2026",
            source_main_title: "Information security, cybersecurity and privacy protection",
            source_subtitle_prefix: "- Guidelines on privacy preservation based on",
            source_subtitle_highlight: "  Zero-Knowledge Proofs",
            source_url: "https://www.iso.org/standard/80398.html",
            source_link_label: "자료: ISO 공식 사이트",
            image_src: "/assets/iso-standard-dossier.png",
            image_alt: "ISO 표준 문서를 연상시키는 ZKP 프라이버시 검증 자료 이미지",
        },
        use_cases: vec![
            UseCase { title: "가상자산 준법·감사", detail: "잔고대사, 감사 대응 증적자료", class_name: "orbit-top", tone_class: "case-finance case-asset", icon: "audit" },
            UseCase { title: "투표·공공 신뢰", detail: "절차와 결과의 공정성 확보", class_name: "orbit-right-top", tone_class: "case-trust", icon: "vote" },
            UseCase { title: "디지털 신원", detail: "필요한 정보만 선택적 공개", class_name: "orbit-right-bottom", tone_class: "case-trust", icon: "id" },
            UseCase { title: "자격·연령 확인", detail: "기준 충족 여부만 확인", class_name: "orbit-bottom", tone_class: "case-trust", icon: "age" },
            UseCase { title: "공정추첨·경매", detail: "규칙대로 했는지 결과 검증", class_name: "orbit-left-bottom", tone_class: "case-finance case-lottery", icon: "lottery" },
            UseCase { title: "CBDC·기관형 분산원장", detail: "정보보호 송금, 준비금 검증", class_name: "orbit-left-top", tone_class: "case-finance", icon: "cbdc" },
        ],
        product_gateway: ProductGateway {
            eyebrow: "ZKP Product",
            title: "검증 가능한 신뢰 레이어를 제공합니다.",
            cards: vec![
                ProductCard {
                    name: "zkWallet",
                    href: "/products/zkwallet",
                    cta_label: "더 보기",
                    tone_class: "product-card-wallet",
                    image_src: "/assets/zkwallet-product-slide-1.png",
                    image_alt: "zkWallet 제품소개서 첫 페이지 이미지",
                    external: false,
                },
                ProductCard {
                    name: "zkPoRL",
                    href: "/products/zkporl",
                    cta_label: "더 보기",
                    tone_class: "product-card-porl",
                    image_src: "/assets/zkporl-product-slide-1.png",
                    image_alt: "zkPoRL 제품소개서 첫 페이지 이미지",
                    external: false,
                },
                ProductCard {
                    name: "zkVoting",
                    href: "/products/zkvoting",
                    cta_label: "더 보기",
                    tone_class: "product-card-voting",
                    image_src: "/assets/zkvoting-product-slide-2.png",
                    image_alt: "zkVoting 제품소개서 스타일 초안 이미지",
                    external: true,
                },
            ],
        },
        trust_proof: TrustProof {
            eyebrow: "Trust",
            headline: "ZKRYPTO History",
            lead: "ZKP 기술 기반 신뢰 레이어를 설계합니다.",
            milestones: vec![
                ProofMilestone { phase: "Research", title: "ZKP 연구 기반 역량 축적", body: "프라이버시를 지키면서 사실을 검증하는 암호 기술을 연구해왔습니다." },
                ProofMilestone { phase: "Standard", title: "ISO/IEC 27565:2026 ZKP 기반 서비스", body: "데이터 최소화, 선택적 공개의 원칙을 서비스에 구현했습니다." },
                ProofMilestone { phase: "Product", title: "Fineapple 신뢰 인프라 설계", body: "디지털 자산의 발행, 잔고증명에서 감사까지 설명 가능한 흐름으로 연결합니다." },
                ProofMilestone { phase: "Expansion", title: "공공·금융 적용 시나리오 확장", body: "신원, 자격, 투표, CBDC 등 적용 가능 영역을 구체화하고 있습니다." },
            ],
            proof_items: vec![
                ProofItem { label: "Public", title: "CES 2024 최고혁신상 수상", detail: "zkVoting Poll Station, Cybersecurity & Personal Privacy 분야 Best of Innovation.", tone_class: "proof-public" },
                ProofItem { label: "Award", title: "CES 2023·2024 2년 연속 최고혁신상", detail: "zkVoting, zkWallet, zkVoting Poll Station 관련 수상 이력.", tone_class: "proof-award" },
                ProofItem { label: "IP", title: "특허 11건 등록, 17건 출원", detail: "국내 4건, 해외 7건 등록으로 정리된 ZKP R&D·IP 포트폴리오.", tone_class: "proof-ip" },
                ProofItem { label: "Research", title: "최근 5년 18개 논문 게재", detail: "zkMarket, zkAML, Aegis, vCNN, zkLogis, zkVoting, Azeroth 등 ZKP 연구 성과.", tone_class: "proof-research" },
                ProofItem { label: "CBDC", title: "한국은행 CBDC 파일럿 프로젝트 수행", detail: "프라이버시 보호 CBDC, AML/CFT, 성능 검증 관련 이력.", tone_class: "proof-public" },
                ProofItem { label: "Public Trust", title: "중앙선거관리위원회 블록체인 투표 적용", detail: "zkVoting Poll Station 기반 공공 투표 적용 이력.", tone_class: "proof-public" },
                ProofItem { label: "Certification", title: "ISO 9001·KISA 신속확인", detail: "품질·보안 신뢰 인프라와 제품 적용 가능성을 보여주는 대외 증빙 항목입니다.", tone_class: "proof-cert" },
                ProofItem { label: "Award", title: "과기정통부장관상·모바일 기술대상", detail: "기술성과 제품 경쟁력을 함께 보여주는 수상 이력입니다.", tone_class: "proof-award" },
            ],
        },
        final_cta: FinalCta {
            eyebrow: "Next",
            title: "기존 서비스에 신뢰 레이어를 추가하세요.",
            body: "디지털 자산, 공공 신뢰인프라, 연구 협업 등",
            primary: Cta { label: "제품 보기", href: "#products" },
            secondary: Cta { label: "문의 하기", href: "#contact" },
            tertiary: Cta { label: "자료 요청", href: "#contact" },
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
        footer_text: "표준 기반 ZKP로 디지털 자산과 공공 데이터를 위한 신뢰 검증 인프라를 구축합니다.",
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
        title: "zkPoRL | Fineapple 고객부채·준비자산 검증 인프라",
        description: "zkPoRL은 거래소, 수탁사, 은행이 고객부채와 준비자산의 정합성을 더 짧은 주기로 확인하도록 돕는 검증 인프라입니다.",
        product_name: "zkPoRL",
        eyebrow: "Fineapple 금융 제품군",
        hero_title: "고객부채와 준비자산을 짧은 주기로 맞춰 보는 검증 인프라",
        hero_lead: "개별 잔고를 공개하지 않고도, 원장과 자산 상태가 맞는지 확인할 수 있게 돕습니다.",
        hero_note: " ",
        primary_cta: Cta { label: "도입 상담하기", href: "/#contact" },
        secondary_cta: Cta { label: "제품군으로 돌아가기", href: "/#products" },
        stats: vec![
            Metric { value: "Privacy", label: "정보보호", detail: "민감정보를 외부에 직접 노출하지 않는 검증 구조" },
            Metric { value: "Verifiability", label: "검증", detail: "정의된 입력과 검증 명제의 수학적 정합성 확인" },
            Metric { value: "Operations", label: "운영", detail: "증명 결과를 운영 대시보드, 로그, 증적 자료로 연결" },
        ],
        problem_title: "스냅샷과 후행점검만으로는 운영 중 리스크를 설명하기 어렵습니다",
        problem_body: "정기 감사와 특정 시점 리포트는 필요하지만, 점검 사이에 발생하는 장부 불일치, 기준시점 차이, 누락·중복 이벤트를 빠르게 포착하기 어렵습니다.",
        problems: vec![
            Feature { title: "대사 주기", body: "일·월·분기 단위 확인" },
            Feature { title: "기준 시점", body: "원장·지갑·온체인 대사 시점 불일치" },
            Feature { title: "민감정보", body: "고객 잔고·거래내역 직접 공개 곤란" },
            Feature { title: "감사 대응", body: "자료 분산·반복 취합 부담" },
        ],
        answer_title: "zkPoRL은 원장·지갑·온체인 상태를 하나의 확인 흐름으로 묶습니다.",
        answer_body: "원천 시스템을 대신하지 않습니다. 필요한 데이터를 읽고 정렬한 뒤, 정합성 확인 결과와 조치 기록을 남기도록 돕습니다.",
        answer_cards: vec![
            Feature { title: "PoL 상시 대사", body: "고객부채 합계가 기준 데이터와 맞는지 짧은 주기로 확인합니다." },
            Feature { title: "민감정보 보호", body: "개별 잔고를 직접 보여주지 않고 필요한 확인 결과 중심으로 설명합니다." },
            Feature { title: "이상징후 Case", body: "누락, 중복, 차이 후보를 운영자가 확인할 조치 항목으로 남깁니다." },
            Feature { title: "증빙 패키지", body: "검증 실행 결과, 기준시점 정보, 조치 이력을 감사 참고자료로 묶습니다." },
        ],
        workflow_title: "데이터 확인부터 조치 기록까지 같은 화면 흐름으로 이어집니다.",
        workflow_body: "운영자는 증명값 자체보다 오늘 확인해야 할 차이, 원인 후보, 조치 상태를 빠르게 보는 것이 중요합니다.",
        workflow: vec![
            ProcessItem { number: "01", title: "데이터 읽기", body: "전산원장, 지갑 잔고, 온체인 상태를 읽습니다." },
            ProcessItem { number: "02", title: "기준 맞추기", body: "자산 단위와 기준시점을 맞춰 비교 가능한 형태로 정리합니다." },
            ProcessItem { number: "03", title: "PoL 확인", body: "고객부채 합계와 기준 데이터의 정합성을 확인합니다." },
            ProcessItem { number: "04", title: "이상징후 분류", body: "누락, 중복, 차이 후보를 조치할 항목으로 분리합니다." },
            ProcessItem { number: "05", title: "증빙 정리", body: "검증 결과와 조치 이력을 감사 참고자료로 묶습니다." },
        ],
        operations_title: "데모는 PoL 업무 화면을 중심으로 확인합니다.",
        operations_body: "데모 화면은 상담 미팅에서 상시 대사와 이상징후 조치 흐름을 함께 확인하는 방식으로 운영합니다.",
        operations: vec![
            Feature { title: "ZP-1 상시 대사", body: "고객부채 합계를 개별 잔고 비공개 상태로 확인하고 대시보드에서 상태를 봅니다." },
            Feature { title: "ZP-4 이상징후 조치", body: "중복 유입이나 차이 후보를 확인하고 증명 실패, 지급 제한 검토 등 조치 기록으로 연결합니다." },
            Feature { title: "운영자 대시보드", body: "검증 상태, 차이 후보, Case 진행 상태를 운영자가 한 화면에서 확인합니다." },
            Feature { title: "PoR 확장", body: "7월 로드맵에서 준비자산 검증을 결합해 zkPoRL 흐름으로 넓힙니다." },
        ],
        proof_title: "감사 의견을 대신하지 않고, 감사에 필요한 근거를 정리합니다.",
        proof_body: "zkPoRL은 규제 대응을 보장하는 문구보다, 운영 중 확인한 사실과 조치 기록을 일관되게 남기는 데 초점을 둡니다.",
        proof_points: vec![
            Feature { title: "검증 실행 요약", body: "어느 기간, 어떤 데이터 기준으로 확인했는지 남깁니다." },
            Feature { title: "기준시점 기록", body: "비교에 사용한 데이터 시점과 무결성 근거를 남깁니다." },
            Feature { title: "Alert / Case", body: "이상징후 후보와 담당자 조치 이력을 연결합니다." },
            Feature { title: "공시·감사 참고자료", body: "외부 설명에 필요한 요약 자료를 반복 생성할 수 있게 돕습니다." },
        ],
        cta_title: "PoL 상시 대사부터 PoR 결합 로드맵까지 함께 검토하세요.",
        cta_body: "거래소, 수탁사, 은행의 현재 데이터 구조를 기준으로 PoL 적용 범위와 PoR 확장 시점을 나눠 설계할 수 있습니다.",
    }
}

fn zkwallet_landing() -> ProductLanding {
    ProductLanding {
        theme_class: "product-theme-wallet",
        title: "zkWallet Custody | MPC 기반 고보안 디지털자산 지갑",
        description: "zkWallet Custody는 개인키를 단일 지점에 보관하지 않는 MPC 기반 키 관리·서명 Wallet Service입니다.",
        product_name: "zkWallet Custody : MPC 기반 고보안 Wallet Service",
        eyebrow: "zkWallet Custody",
        hero_title: "키를 한 곳에 모으지 않는 기관형 디지털자산 지갑",
        hero_lead: "zkWallet Custody는 개인키를 여러 조각으로 나눠 보관하고, 정책 검증을 거친 거래만 여러 노드가 공동 서명하는 MPC 기반 고보안 Wallet Service입니다.",
        hero_note: "키 보관을 넘어 생성·갱신·복구·폐기와 기관형 통제까지 제공합니다.",
        primary_cta: Cta { label: "Contact", href: "/#contact" },
        secondary_cta: Cta { label: "제품군으로 돌아가기", href: "/#products" },
        stats: vec![
            Metric { value: "분산 키 서명", label: "MPC", detail: "키 원본 없이 (t/n) 공동 서명" },
            Metric { value: "전 주기 관리", label: "키 생애주기", detail: "생성·갱신·복구·폐기" },
            Metric { value: "정책·감사", label: "기관형 통제", detail: "승인·추적·고가용성" },
        ],
        problem_title: "단일 마스터 시드 구조는 키 하나가 뚫리면 전체 자산이 위험합니다.",
        problem_body: "디지털자산 지갑에서 개인키는 곧 자산 접근 권한입니다. 하나의 니모닉·마스터 시드가 유출되면 전체 자산 탈취로 이어질 수 있습니다.",
        problems: vec![
            Feature { title: "단일 키 유출", body: "마스터 시드 하나의 유출이 전체 자산 탈취로 이어집니다." },
            Feature { title: "권한 오남용", body: "발행·출금 권한이 집중되면 무단 실행 위험이 커집니다." },
            Feature { title: "통제 부재", body: "고위험 작업이 승인·정책 검증 없이 실행될 수 있습니다." },
            Feature { title: "추적·복구 곤란", body: "문제 원인 파악과 장애·키 복구가 어렵습니다." },
        ],
        answer_title: "금융 서비스와 블록체인 사이에서 키 관리·서명을 안전하게 처리합니다.",
        answer_body: "금융 서비스의 키 생성·거래 서명 요청을 받아 MPC로 처리하고, 서명된 거래를 블록체인에 전송합니다.",
        answer_cards: vec![
            Feature { title: "키 관리", body: "키를 생성하고 잠금·해제를 관리합니다." },
            Feature { title: "지갑 관리", body: "지갑을 생성하고 주소를 발급합니다." },
            Feature { title: "서명 관리", body: "서명을 생성하고 검증합니다." },
            Feature { title: "정책 관리", body: "거래 한도와 출금 승인 정책을 적용합니다." },
            Feature { title: "MPC 운영 관리", body: "서명 세션과 참여 노드를 관리합니다." },
            Feature { title: "보안 저장소 관리", body: "Secret과 Master Key를 안전하게 보관합니다." },
        ],
        workflow_title: "정책 검증을 거친 거래만 여러 노드가 공동 서명합니다.",
        workflow_body: "키 조각 A·B·C가 부분 서명에 참여하고, Manager가 이를 결합해 최종 서명을 생성합니다.",
        workflow: vec![
            ProcessItem { number: "01", title: "거래 서명 요청", body: "금융 서비스로부터 거래 서명 요청을 받습니다." },
            ProcessItem { number: "02", title: "정책 검증·승인", body: "정책을 검증한 뒤 서명 절차로 진행합니다." },
            ProcessItem { number: "03", title: "MPC 공동 서명", body: "Party Node A·B·C가 부분 서명을 생성합니다." },
            ProcessItem { number: "04", title: "서명 결합", body: "Manager가 부분 서명을 모아 서명을 생성합니다." },
            ProcessItem { number: "05", title: "결과 반환", body: "생성된 서명 결과를 금융 서비스로 반환합니다." },
        ],
        operations_title: "고객군별 시나리오로 도입 흐름을 확인합니다.",
        operations_body: "개인 사용자, 수탁사, 발행사, 금융사 시나리오로 지갑 개설부터 거래·발행 흐름을 확인합니다.",
        operations: vec![
            Feature { title: "개인 사용자", body: "앱에서 지갑을 개설하고, 전송 시 MPC 서명으로 거래합니다." },
            Feature { title: "수탁사", body: "수탁 지갑을 개설하고 다자 승인 기반으로 입·출금을 처리합니다." },
            Feature { title: "스테이블코인 발행사", body: "준비금 확인과 승인 절차를 거쳐 Mint/Burn을 실행합니다." },
            Feature { title: "금융사", body: "슈퍼앱에 Wallet Service를 연계해 디지털자산 서비스를 확장합니다." },
        ],
        proof_title: "서명 기능을 넘어, 키 생애주기·실행환경·통제를 결합합니다.",
        proof_body: "기관형 Wallet Service는 서명만으로 충분하지 않습니다. zkWallet Custody는 키 생애주기 관리, 보호 실행 환경, 기관형 통제를 함께 제공합니다.",
        proof_points: vec![
            Feature { title: "키 생애주기 관리", body: "생성(DKG)·갱신(Proactive Refresh)·복구·폐기를 통합 관리합니다." },
            Feature { title: "보호 실행 환경", body: "Confidential Computing과 외부 KMS/HSM 연동으로 키 연산을 보호합니다." },
            Feature { title: "MPC 분산 서명", body: "키 조각이 한곳에 모이지 않는 (t/n) 임계치 공동 서명을 수행합니다." },
            Feature { title: "기관형 통제", body: "Policy Engine, Identifiable Abort, 고가용성 구조로 운영을 통제합니다." },
        ],
        cta_title: "도입 환경에 맞는 Wallet Service 제공 방식을 함께 설계하세요.",
        cta_body: "설치형(서버·클라우드)과 Confidential Computing 서버 일체형 중 보안 요구수준에 맞는 방식을 검토합니다.",
    }
}

fn zkvoting_landing() -> ProductLanding {
    ProductLanding {
        theme_class: "product-theme-voting",
        title: "zkVoting | 영지식증명 기반 비밀투표·검증 시스템",
        description: "zkVoting은 개별 표를 공개하지 않고도 집계 결과를 누구나 검증할 수 있는 ZKP 기반 전자투표 솔루션입니다.",
        product_name: "zkVoting : Verifiable Private Voting",
        eyebrow: "zkVoting",
        hero_title: "비밀은 지키고, 결과는 증명하는 투표",
        hero_lead: "zkVoting은 개별 표를 공개하지 않고도 집계 결과의 정합성을 누구나 검증할 수 있게 합니다.",
        hero_note: "투표 비밀성과 결과 검증가능성을 동시에 보장합니다.",
        primary_cta: Cta { label: "Contact", href: "/#contact" },
        secondary_cta: Cta { label: "제품군으로 돌아가기", href: "/#products" },
        stats: vec![
            Metric { value: "비밀투표", label: "프라이버시", detail: "개별 표 비공개" },
            Metric { value: "공개 검증", label: "무결성", detail: "집계 결과 검증 가능" },
            Metric { value: "1인 1표", label: "자격 검증", detail: "이중·부정 투표 차단" },
        ],
        problem_title: "전자투표는 비밀성과 검증가능성을 동시에 충족하기 어렵습니다.",
        problem_body: "투명하게 공개하면 비밀투표가 깨지고, 비공개로 운영하면 결과를 신뢰하기 어렵습니다.",
        problems: vec![
            Feature { title: "비밀 보장", body: "누가 무엇에 투표했는지 드러나면 안 됩니다." },
            Feature { title: "결과 검증", body: "집계가 조작되지 않았음을 외부가 확인해야 합니다." },
            Feature { title: "자격 확인", body: "유권자 자격과 1인 1표를 보장해야 합니다." },
            Feature { title: "운영 신뢰", body: "운영기관 발표에만 의존하기 어렵습니다." },
        ],
        answer_title: "zkVoting은 표를 공개하지 않고 결과만 증명합니다.",
        answer_body: "유권자 자격 검증, 암호화 투표, 영지식 집계 증명, 공개 검증을 하나의 흐름으로 제공합니다.",
        answer_cards: vec![
            Feature { title: "유권자 등록", body: "자격을 검증하고 익명 자격 증명을 발급합니다." },
            Feature { title: "비밀 투표", body: "표를 암호화해 내용을 비공개로 제출합니다." },
            Feature { title: "영지식 집계", body: "복호화 없이 집계 정합성을 증명합니다." },
            Feature { title: "공개 검증", body: "누구나 결과 증명을 검증할 수 있습니다." },
        ],
        workflow_title: "등록부터 검증까지 하나의 검증 가능한 흐름",
        workflow_body: "각 단계가 암호학적 증거로 연결되어 사후 검증이 가능합니다.",
        workflow: vec![
            ProcessItem { number: "01", title: "유권자 등록", body: "자격을 확인하고 익명 자격 증명을 발급합니다." },
            ProcessItem { number: "02", title: "비밀 투표", body: "내용을 암호화한 표를 제출합니다." },
            ProcessItem { number: "03", title: "무결성 검증", body: "중복·부정 투표 후보를 차단합니다." },
            ProcessItem { number: "04", title: "영지식 집계", body: "표를 공개하지 않고 결과를 계산·증명합니다." },
            ProcessItem { number: "05", title: "결과 공개·검증", body: "결과와 증명을 공개해 검증할 수 있게 합니다." },
        ],
        operations_title: "데모는 투표 운영 화면을 중심으로 확인합니다.",
        operations_body: "데모는 상담 미팅에서 투표 생성부터 결과 검증까지 함께 확인하는 방식으로 운영합니다.",
        operations: vec![
            Feature { title: "투표 생성", body: "후보·문항과 유권자 명부를 설정합니다." },
            Feature { title: "투표 진행", body: "유권자 인증과 암호화 투표를 운영합니다." },
            Feature { title: "집계·검증", body: "영지식 집계 결과와 증명을 확인합니다." },
            Feature { title: "감사·공개", body: "검증용 자료를 외부에 공개합니다." },
        ],
        proof_title: "결과 발표를 대신하지 않고, 누구나 검증할 근거를 제공합니다.",
        proof_body: "zkVoting은 신뢰 선언보다, 암호학적으로 검증 가능한 증거를 일관되게 남기는 데 초점을 둡니다.",
        proof_points: vec![
            Feature { title: "유권자 자격 증명", body: "자격과 1인 1표를 익명으로 보장합니다." },
            Feature { title: "투표 비밀성", body: "개별 표의 내용을 공개하지 않습니다." },
            Feature { title: "집계 정합성 증명", body: "결과가 제출된 표와 일치함을 증명합니다." },
            Feature { title: "공개 검증 자료", body: "누구나 결과를 검증할 수 있게 공개합니다." },
        ],
        cta_title: "검증 가능한 비밀투표를 직접 확인해보세요.",
        cta_body: "선거, 총회, 거버넌스 등 적용 시나리오에 맞춰 PoC 범위를 함께 설계할 수 있습니다.",
    }
}
