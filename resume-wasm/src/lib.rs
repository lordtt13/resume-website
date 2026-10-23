use wasm_bindgen::prelude::*;
use pdf_writer::{Content, Name, Pdf, Rect, Ref, Str, Finish};
use pdf_writer::writers::Annotation;
use pdf_writer::types::{AnnotationType, ActionType};
use js_sys::Uint8Array;

// ---------------------------
// Page geometry (A4, points)
// ---------------------------
const PAGE_WIDTH: f32 = 595.0;
const PAGE_HEIGHT: f32 = 842.0;
const MARGIN: f32 = 50.0;
const RIGHT: f32 = PAGE_WIDTH - MARGIN;
#[cfg(test)]
const BOTTOM_MARGIN: f32 = 36.0;

const BODY_SIZE: f32 = 9.5;
const LEADING: f32 = 12.0;

const REGULAR: Name = Name(b"F1");
const BOLD: Name = Name(b"F2");

const BLACK: [f32; 3] = [0.1, 0.1, 0.1];
const GRAY: [f32; 3] = [0.4, 0.4, 0.4];
const ACCENT: [f32; 3] = [0.23, 0.51, 0.96];

// Helvetica glyph widths (1/1000 em) for ASCII 32..=126, from the standard AFM.
const HELVETICA_WIDTHS: [u16; 95] = [
    278, 278, 355, 556, 556, 889, 667, 222, 333, 333, 389, 584, 278, 333, 278, 278,
    556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 278, 278, 584, 584, 584, 556,
    1015, 667, 667, 722, 722, 667, 611, 778, 722, 278, 500, 667, 556, 833, 722, 778,
    667, 778, 722, 667, 611, 722, 667, 944, 667, 667, 611, 278, 278, 278, 469, 556,
    222, 556, 556, 500, 556, 556, 278, 556, 556, 222, 222, 500, 222, 833, 556, 556,
    556, 556, 333, 500, 278, 556, 500, 722, 500, 500, 500, 334, 260, 334, 584,
];

fn text_width(text: &str, size: f32) -> f32 {
    let units: u32 = text
        .bytes()
        .map(|b| match b {
            32..=126 => HELVETICA_WIDTHS[(b - 32) as usize] as u32,
            _ => 556,
        })
        .sum();
    units as f32 * size / 1000.0
}

fn wrap(text: &str, size: f32, max_width: f32) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if line.is_empty() {
            line.push_str(word);
        } else if text_width(&line, size) + text_width(" ", size) + text_width(word, size) > max_width {
            lines.push(line);
            line = String::from(word);
        } else {
            line.push(' ');
            line.push_str(word);
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

// ---------------------------
// Resume content
// ---------------------------
const SUMMARY: &str = "Senior backend and infrastructure engineer with 5+ years of experience designing, \
building and operating production systems. Owns architecture and technical direction across AI, data and \
backend platforms, from requirements and build-vs-buy decisions through production operations. Built systems \
processing 22M+ records/day and LLM pipelines processing data from 6,000+ companies.";

struct Role {
    title: &'static str,
    org: &'static str,
    dates: &'static str,
    bullets: &'static [&'static str],
}

const EXPERIENCE: &[Role] = &[
    Role {
        title: "Software Engineer",
        org: "Sustainable Lab Inc., Tokyo, Japan",
        dates: "Nov 2024 - Present",
        bullets: &[
            "Owned architecture and technical direction for the company's AI, QA and data automation products, \
             from requirements and build-vs-buy decisions through production and on-call; held AWS spend flat at \
             ~$1.5K/month for the entire stack despite daily data growth by moving workloads to trigger-based \
             Lambda functions and Glue jobs.",
            "Designed and built an LLM-based document-parsing system for corporate ESG disclosures of 6,000+ \
             companies, processing ~2,000 documents/month and entirely replacing manual parsing.",
            "Built internal QA tooling to validate ~1M new records/month, cutting manual review time from \
             40-45 days to 7 days.",
            "Engineered ETL automation (scheduling, retries, monitoring) and established observability and \
             alerting practices, keeping 30 pipelines processing 22M+ records/day running smoothly.",
            "Led technical design discussions across a team of up to 5 engineers, reviewed 10+ PRs/month, \
             defined coding and testing standards, broke down and estimated work with product and data teams, \
             and conducted technical interviews.",
        ],
    },
    Role {
        title: "Software Engineer",
        org: "Goalist India Pvt. Ltd., Bengaluru, India",
        dates: "Feb 2022 - Oct 2024",
        bullets: &[
            "Delivered backend systems for 6+ production products across Java, Python, Ruby, Go and TypeScript \
             within a team of 5+ engineers.",
            "Designed and built the movie-work.jp job platform backend with Java and Spring Boot.",
            "Led the booking system's infrastructure migration for the CRIE renewal, and built the Django backend \
             and deployment pipeline for b-align.dental.",
            "Maintained and evolved the Ruby on Rails microservices architecture (6 services) for PowerWork Pro.",
            "Deployed an invoice management system on AWS and Kubernetes using NestJS and React.",
            "Built webhook-triggered ETL pipelines and external data APIs.",
            "Developed a test-driven Go microservice for HRBC reminders and internal tools built on OpenAI APIs.",
        ],
    },
];

const PROJECTS: &[&str] = &[
    "Hugging Face Transformers (community contributor) - Authored two notebooks listed in the official \
     Transformers community docs: fine-tuning BlenderBotSmall for summarization with the Trainer API, and \
     expanding SciBERT's vocabulary on the CORD-19 dataset.",
    "CloudFormation Generator (Go, IaC) - Built a tool that automates generation of AWS CloudFormation templates.",
    "Node TS GraphQL Template (TypeScript, GraphQL, PostgreSQL) - Created a production-ready starter template \
     for Node.js GraphQL services.",
];

const SKILLS: &[(&str, &str)] = &[
    ("Backend", "Spring Boot, Django, Rails, NestJS, REST, GraphQL, Microservices"),
    ("Cloud", "AWS (Lambda, ECS, Glue), Kubernetes, Docker, Terraform, CI/CD"),
    ("Databases", "PostgreSQL, MySQL, Redis, DynamoDB"),
    ("Data / AI", "ETL, LLM pipelines, OpenAI APIs"),
    ("Languages", "Java, Go, Python, TypeScript, Ruby"),
];

const CONTACT: &[(&str, Option<&str>)] = &[
    ("Tokyo, Japan", None),
    ("tanmaythakur.dev@gmail.com", Some("mailto:tanmaythakur.dev@gmail.com")),
    ("github.com/lordtt13", Some("https://github.com/lordtt13")),
    ("LinkedIn", Some("https://www.linkedin.com/in/tanmay-thakur-6bb5a9154/")),
];

// ---------------------------
// Layout helpers
// ---------------------------
struct Layout {
    content: Content,
    y: f32,
    links: Vec<(Rect, &'static str)>,
}

impl Layout {
    fn text(&mut self, font: Name, size: f32, x: f32, color: [f32; 3], text: &str) {
        let [r, g, b] = color;
        self.content.set_fill_rgb(r, g, b);
        self.content.begin_text();
        self.content.set_font(font, size);
        self.content.set_text_matrix([1.0, 0.0, 0.0, 1.0, x, self.y]);
        self.content.show(Str(text.as_bytes()));
        self.content.end_text();
    }

    fn paragraph(&mut self, x: f32, text: &str) {
        for line in wrap(text, BODY_SIZE, RIGHT - x) {
            self.text(REGULAR, BODY_SIZE, x, BLACK, &line);
            self.y -= LEADING;
        }
    }

    fn bullet(&mut self, text: &str) {
        self.text(REGULAR, BODY_SIZE, MARGIN + 4.0, BLACK, "-");
        self.paragraph(MARGIN + 14.0, text);
        self.y -= 2.0;
    }

    fn section(&mut self, title: &str) {
        self.y -= 8.0;
        self.text(BOLD, 11.0, MARGIN, ACCENT, title);
        self.y -= 5.0;
        self.content.save_state();
        let [r, g, b] = ACCENT;
        self.content.set_stroke_rgb(r, g, b);
        self.content.set_line_width(0.75);
        self.content.move_to(MARGIN, self.y);
        self.content.line_to(RIGHT, self.y);
        self.content.stroke();
        self.content.restore_state();
        self.y -= 14.0;
    }

    fn role(&mut self, role: &Role) {
        self.text(BOLD, 10.0, MARGIN, BLACK, &format!("{} - {}", role.title, role.org));
        self.text(REGULAR, BODY_SIZE, RIGHT - text_width(role.dates, BODY_SIZE), GRAY, role.dates);
        self.y -= 14.0;
        for bullet in role.bullets {
            self.bullet(bullet);
        }
        self.y -= 4.0;
    }
}

/// Lays out the resume and returns the PDF bytes along with the final cursor height,
/// so tests can check the content still fits on one page.
fn render() -> (Vec<u8>, f32) {
    let mut l = Layout { content: Content::new(), y: 790.0, links: Vec::new() };

    // Header
    l.text(BOLD, 22.0, MARGIN, BLACK, "Tanmay Thakur");
    l.y -= 20.0;
    l.text(REGULAR, 12.0, MARGIN, GRAY, "Senior Backend & Infrastructure Engineer");
    l.y -= 16.0;

    let mut x = MARGIN;
    for (i, (label, uri)) in CONTACT.iter().enumerate() {
        if i > 0 {
            l.text(REGULAR, BODY_SIZE, x, GRAY, "  |  ");
            x += text_width("  |  ", BODY_SIZE);
        }
        let width = text_width(label, BODY_SIZE);
        match uri {
            Some(uri) => {
                l.text(REGULAR, BODY_SIZE, x, ACCENT, label);
                l.links.push((Rect::new(x, l.y - 2.0, x + width, l.y + 9.0), uri));
            }
            None => l.text(REGULAR, BODY_SIZE, x, GRAY, label),
        }
        x += width;
    }
    l.y -= 14.0;

    l.section("SUMMARY");
    l.paragraph(MARGIN, SUMMARY);

    l.section("EXPERIENCE");
    for role in EXPERIENCE {
        l.role(role);
    }

    l.section("PROJECTS & OPEN SOURCE");
    for project in PROJECTS {
        l.bullet(project);
    }

    l.section("SKILLS");
    for (label, value) in SKILLS {
        l.text(BOLD, BODY_SIZE, MARGIN, BLACK, label);
        l.text(REGULAR, BODY_SIZE, MARGIN + 75.0, BLACK, value);
        l.y -= LEADING;
    }

    l.section("EDUCATION");
    let dates = "2017 - 2021";
    l.text(BOLD, 10.0, MARGIN, BLACK, "Vellore Institute of Technology");
    l.text(REGULAR, BODY_SIZE, RIGHT - text_width(dates, BODY_SIZE), GRAY, dates);
    l.y -= LEADING;
    l.text(REGULAR, BODY_SIZE, MARGIN, BLACK, "B.Tech, Electrical and Electronics Engineering");

    let Layout { content, y: final_y, links } = l;

    // ---------------------------
    // Document structure
    // ---------------------------
    let mut pdf = Pdf::new();
    let catalog_id = Ref::new(1);
    let pages_id = Ref::new(2);
    let page_id = Ref::new(3);
    let font_id = Ref::new(4);
    let bold_font_id = Ref::new(5);
    let content_id = Ref::new(6);
    let annot_ids: Vec<Ref> = (0..links.len()).map(|i| Ref::new(7 + i as i32)).collect();

    pdf.catalog(catalog_id).pages(pages_id);
    pdf.pages(pages_id).count(1).kids([page_id]);

    let mut page = pdf.page(page_id);
    page.media_box(Rect::new(0.0, 0.0, PAGE_WIDTH, PAGE_HEIGHT));
    page.parent(pages_id);
    page.contents(content_id);
    page.resources().fonts().pair(REGULAR, font_id).pair(BOLD, bold_font_id);
    page.insert(Name(b"Annots")).array().items(annot_ids.iter().copied());
    page.finish();

    // Base 14 fonts, no embedding needed
    pdf.type1_font(font_id).base_font(Name(b"Helvetica")).finish();
    pdf.type1_font(bold_font_id).base_font(Name(b"Helvetica-Bold")).finish();

    pdf.stream(content_id, &content.finish());

    for (id, (rect, uri)) in annot_ids.iter().zip(links) {
        pdf.indirect(*id)
            .start::<Annotation>()
            .subtype(AnnotationType::Link)
            .rect(rect)
            .action()
            .action_type(ActionType::Uri)
            .uri(Str(uri.as_bytes()))
            .finish()
            .finish();
    }

    (pdf.finish(), final_y)
}

#[wasm_bindgen]
pub fn generate_resume_pdf() -> Uint8Array {
    let (buffer, _) = render();
    Uint8Array::from(buffer.as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resume_fits_on_one_page() {
        let (pdf, final_y) = render();
        assert!(final_y >= BOTTOM_MARGIN, "content overflows the page (y = {final_y})");
        if let Ok(path) = std::env::var("RESUME_PDF_OUT") {
            std::fs::write(path, pdf).unwrap();
        }
    }

    #[test]
    fn content_is_ascii() {
        // Base 14 fonts use StandardEncoding, so non-ASCII characters would render as garbage.
        let all = EXPERIENCE.iter().flat_map(|r| r.bullets.iter().copied().chain([r.title, r.org, r.dates]))
            .chain(PROJECTS.iter().copied())
            .chain(SKILLS.iter().flat_map(|(a, b)| [*a, *b]))
            .chain([SUMMARY]);
        for text in all {
            assert!(text.is_ascii(), "non-ASCII text: {text}");
        }
    }
}
