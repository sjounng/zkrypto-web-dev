mod content;

use askama::Template;
use axum::{
    extract::Form,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router,
};
use content::{product_landing, site_content, ProductLanding, SiteContent};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate<'a> {
    content: &'a SiteContent,
    contact: ContactView,
}

#[derive(Template)]
#[template(path = "product_zkporl.html")]
struct ZkporlTemplate<'a> {
    content: &'a SiteContent,
    product: &'a ProductLanding,
}

#[derive(Template)]
#[template(path = "product_zkwallet.html")]
struct ZkwalletTemplate<'a> {
    content: &'a SiteContent,
    product: &'a ProductLanding,
}

#[derive(Template)]
#[template(path = "product_zkvoting.html")]
struct ZkvotingTemplate<'a> {
    content: &'a SiteContent,
    product: &'a ProductLanding,
}

#[derive(Debug, Deserialize)]
struct ContactForm {
    email: String,
    message: String,
    privacy: Option<String>,
}

#[derive(Clone, Debug)]
struct ContactView {
    email: String,
    message: String,
    privacy_checked: bool,
    show_feedback: bool,
    feedback_class: &'static str,
    feedback_message: String,
}

impl Default for ContactView {
    fn default() -> Self {
        Self {
            email: String::new(),
            message: String::new(),
            privacy_checked: false,
            show_feedback: false,
            feedback_class: "",
            feedback_message: "실제 접수 이메일 또는 CRM 연동은 배포 전 연결 필요".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(home))
        .route("/products/zkporl", get(product_zkporl))
        .route("/products/zkPoRL", get(product_zkporl))
        .route("/products/zkwallet", get(product_zkwallet))
        .route("/products/zkvoting", get(product_zkvoting))
        .route("/contact", post(contact))
        .route("/healthz", get(healthz))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/static", ServeDir::new("static"));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("ZKRYPTO v6 dynamic server listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn home() -> Result<Html<String>, StatusCode> {
    render_home(ContactView::default())
}

async fn contact(Form(form): Form<ContactForm>) -> Result<Html<String>, StatusCode> {
    render_home(validate_contact(form))
}

async fn product_zkporl() -> Result<Html<String>, StatusCode> {
    let content = site_content();
    let product = product_landing("zkporl").ok_or(StatusCode::NOT_FOUND)?;
    render_template(ZkporlTemplate {
        content: &content,
        product: &product,
    })
}

async fn product_zkwallet() -> Result<Html<String>, StatusCode> {
    let content = site_content();
    let product = product_landing("zkwallet").ok_or(StatusCode::NOT_FOUND)?;
    render_template(ZkwalletTemplate {
        content: &content,
        product: &product,
    })
}

async fn product_zkvoting() -> Result<Html<String>, StatusCode> {
    let content = site_content();
    let product = product_landing("zkvoting").ok_or(StatusCode::NOT_FOUND)?;
    render_template(ZkvotingTemplate {
        content: &content,
        product: &product,
    })
}

async fn healthz() -> &'static str {
    "ok"
}

fn render_home(contact: ContactView) -> Result<Html<String>, StatusCode> {
    let content = site_content();
    HomeTemplate {
        content: &content,
        contact,
    }
    .render()
    .map(Html)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn render_template<T: Template>(template: T) -> Result<Html<String>, StatusCode> {
    template
        .render()
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn validate_contact(form: ContactForm) -> ContactView {
    let email = form.email.trim().to_string();
    let message = form.message.trim().to_string();
    let privacy_checked = form.privacy.is_some();
    let mut errors = Vec::new();

    if !is_valid_email(&email) {
        errors.push("이메일 형식을 확인해주세요.");
    }
    if message.is_empty() {
        errors.push("문의 내용을 입력해주세요.");
    }
    if !privacy_checked {
        errors.push("개인정보 수집·이용 동의가 필요합니다.");
    }

    if errors.is_empty() {
        ContactView {
            email: String::new(),
            message: String::new(),
            privacy_checked: false,
            show_feedback: true,
            feedback_class: "success",
            feedback_message:
                "문의가 접수되었습니다. 실제 이메일 또는 CRM 전송은 배포 전 연결해주세요."
                    .to_string(),
        }
    } else {
        ContactView {
            email,
            message,
            privacy_checked,
            show_feedback: true,
            feedback_class: "error",
            feedback_message: errors.join(" "),
        }
    }
}

fn is_valid_email(value: &str) -> bool {
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };

    !local.trim().is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contact_validation_accepts_complete_submission() {
        let result = validate_contact(ContactForm {
            email: "team@example.com".to_string(),
            message: "도입 상담을 요청합니다.".to_string(),
            privacy: Some("yes".to_string()),
        });

        assert_eq!(result.feedback_class, "success");
        assert!(result.feedback_message.contains("문의가 접수되었습니다"));
        assert!(result.email.is_empty());
    }

    #[test]
    fn contact_validation_rejects_empty_submission() {
        let result = validate_contact(ContactForm {
            email: "".to_string(),
            message: "".to_string(),
            privacy: None,
        });

        assert_eq!(result.feedback_class, "error");
        assert!(result.feedback_message.contains("이메일"));
        assert!(result.feedback_message.contains("문의 내용"));
        assert!(result.feedback_message.contains("개인정보"));
    }
}
