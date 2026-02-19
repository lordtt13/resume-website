use wasm_bindgen::prelude::*;
use pdf_writer::{Content, Name, Pdf, Rect, Ref, Str, Finish};
use pdf_writer::writers::Annotation;
use pdf_writer::types::{AnnotationType, ActionType};
use js_sys::Uint8Array;

#[wasm_bindgen]
pub fn generate_resume_pdf() -> Uint8Array {
    let mut pdf = Pdf::new();

    // Object IDs
    let catalog_id = Ref::new(1);
    let pages_id = Ref::new(2);
    let page_id = Ref::new(3);
    let font_id = Ref::new(4);
    let content_id = Ref::new(5);

    let linkedin_annot_id = Ref::new(6);
    let github_annot_id = Ref::new(7);
    let email_annot_id = Ref::new(8);

    // ---------------------------
    // Catalog + Pages
    // ---------------------------
    pdf.catalog(catalog_id).pages(pages_id);
    pdf.pages(pages_id).count(1).kids([page_id]);

    // ---------------------------
    // Page
    // ---------------------------
    let mut page = pdf.page(page_id);
    page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0)); // A4
    page.parent(pages_id);
    page.contents(content_id);
    page.resources().fonts().pair(Name(b"F1"), font_id);

    page.insert(Name(b"Annots")).array().items([linkedin_annot_id, github_annot_id, email_annot_id]);

    page.finish();

    // ---------------------------
    // Font (Base 14 Helvetica)
    // ---------------------------
    pdf.type1_font(font_id)
        .base_font(Name(b"Helvetica"))
        .finish();

    // ---------------------------
    // Content Stream
    // ---------------------------
    let mut content = Content::new();
    let mut y = 800.0;

    macro_rules! draw_text {
        ($size:expr, $x:expr, $text:expr) => {{
            content.begin_text();
            content.set_font(Name(b"F1"), $size);
            content.set_text_matrix([1.0, 0.0, 0.0, 1.0, $x, y]);
            content.show(Str($text));
            content.end_text();
        }};
    }

    macro_rules! draw_line {
        () => {{
            content.save_state();
            content.set_stroke_color([0.8, 0.8, 0.8]);
            content.set_line_width(1.0);
            content.move_to(50.0, y);
            content.line_to(545.0, y);
            content.stroke();
            content.restore_state();
            y -= 25.0; // Space after line
        }};
    }

    // Header
    content.set_fill_color([0.0, 0.0, 0.0]);
    draw_text!(24.0, 50.0, b"Tanmay Thakur");
    y -= 25.0;

    content.set_fill_color([0.4, 0.4, 0.4]);
    draw_text!(14.0, 50.0, b"Backend & Infrastructure Engineer");
    y -= 20.0;

    content.set_fill_color([0.2, 0.2, 0.2]);
    draw_text!(10.0, 50.0, b"Tokyo, Japan");
    y -= 15.0;

    // ---------------------------
    // Links Row
    // ---------------------------
    let link_y = y;

    content.set_fill_color([0.23, 0.51, 0.96]);
    draw_text!(10.0, 50.0, b"tanmaythakur.dev@gmail.com");
    draw_text!(10.0, 220.0, b"LinkedIn");
    draw_text!(10.0, 300.0, b"GitHub");

    y -= 15.0;

    // Divider
    draw_line!();

    // ---------------------------
    // SKILLS
    // ---------------------------
    content.set_fill_color([0.23, 0.51, 0.96]);
    draw_text!(12.0, 50.0, b"SKILLS");
    y -= 20.0;

    content.set_fill_color([0.0, 0.0, 0.0]);
    draw_text!(10.0, 50.0, b"Backend: Microservices, REST APIs, Spring Boot, Django, Go, Ruby on Rails");
    y -= 15.0;
    draw_text!(10.0, 50.0, b"Cloud & DevOps: AWS (Lambda, ECS, Glue), Docker, Kubernetes, Terraform");
    y -= 15.0;
    draw_text!(10.0, 50.0, b"MLOps & Data: ETL pipelines, Webhooks, OpenAI integrations");
    y -= 15.0;

    draw_line!();

    // ---------------------------
    // EXPERIENCE
    // ---------------------------
    content.set_fill_color([0.23, 0.51, 0.96]);
    draw_text!(13.0, 50.0, b"EXPERIENCE");
    y -= 20.0;

    content.set_fill_color([0.0, 0.0, 0.0]);
    draw_text!(12.0, 50.0, b"Sustainable Lab Inc. (Nov 2024 - Present)");
    y -= 15.0;

    draw_text!(10.0, 60.0, b"- Developing TERRAST for Enterprise ESG platform.");
    y -= 15.0;

    draw_text!(10.0, 60.0, b"- Designing scalable backend services.");
    y -= 25.0;

    draw_text!(12.0, 50.0, b"Goalist India Pvt. Ltd. (Feb 2022 - Oct 2024)");
    y -= 15.0;

    draw_text!(10.0, 60.0, b"- Built movie-work.jp job platform (Java/Spring Boot).");
    y -= 15.0;

    draw_text!(10.0, 60.0, b"- Deployed invoice system on AWS/K8s.");
    y -= 15.0;

    draw_text!(10.0, 60.0, b"- Developed Go microservice using TDD.");
    y -= 15.0;

    draw_text!(10.0, 60.0, b"- Engineered Django backend for b-align.dental.");
    y -= 15.0;

    draw_text!(10.0, 60.0, b"- Modernized booking infrastructure for CRIE.");
    y -= 15.0;

    draw_text!(10.0, 60.0, b"- Built Scrapy crawlers & webhook ETL pipelines.");
    y -= 25.0;

    draw_line!();

    // ---------------------------
    // EDUCATION
    // ---------------------------
    content.set_fill_color([0.23, 0.51, 0.96]);
    draw_text!(13.0, 50.0, b"EDUCATION");
    y -= 20.0;

    content.set_fill_color([0.0, 0.0, 0.0]);
    draw_text!(12.0, 50.0, b"Vellore Institute of Technology (2017 - 2021)");
    y -= 15.0;

    draw_text!(10.0, 50.0, b"B.Tech - Electrical and Electronics Engineering");

    // Finish content stream
    let content_bytes = content.finish();
    pdf.stream(content_id, &content_bytes);

    // ---------------------------
    // Link Annotation Objects
    // ---------------------------

    pdf.indirect(email_annot_id)
        .start::<Annotation>()
        .subtype(AnnotationType::Link)
        .rect(Rect::new(50.0, link_y - 2.0, 210.0, link_y + 10.0))
        .action()
        .action_type(ActionType::Uri)
        .uri(Str(b"mailto:tanmaythakur.dev@gmail.com"))
        .finish()
        .finish();

    pdf.indirect(linkedin_annot_id)
        .start::<Annotation>()
        .subtype(AnnotationType::Link)
        .rect(Rect::new(220.0, link_y - 2.0, 290.0, link_y + 10.0))
        .action()
        .action_type(ActionType::Uri)
        .uri(Str(b"https://www.linkedin.com/in/tanmay-thakur-6bb5a9154/"))
        .finish()
        .finish();

    pdf.indirect(github_annot_id)
        .start::<Annotation>()
        .subtype(AnnotationType::Link)
        .rect(Rect::new(300.0, link_y - 2.0, 350.0, link_y + 10.0))
        .action()
        .action_type(ActionType::Uri)
        .uri(Str(b"https://github.com/lordtt13"))
        .finish()
        .finish();

    // ---------------------------
    // Finalize PDF
    // ---------------------------
    let buffer = pdf.finish();

    Uint8Array::from(buffer.as_slice())
}
