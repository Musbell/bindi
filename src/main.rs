use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use askama_actix::Template;
use actix_files as fs;
use serde::Deserialize;
use std::path::Path;
use docx_rust::document::Paragraph;
use docx_rust::Docx;

fn content_and_repeat(content: &str, count: usize, filename: &str) -> Result<String, String> {
    let repeated_content = content.repeat(count);

    // Create a new Word document
    let mut docx = Docx::default();

    // Create a new paragraph with the repeated content
    let para = Paragraph::default().push_text(&*repeated_content);

    // Add the paragraph to the document
    docx.document.push(para);

    // Define the path to the "files" directory
    let path = Path::new("./files").join(filename.to_string() + ".docx");

    // Save the document
    match docx.write_file(&path) {
        Ok(_) => Ok(repeated_content),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[derive(Deserialize)]
pub struct Info {
    content: String,
    count: u32,
    filename: String,
}

async fn generate(req: HttpRequest, info: web::Form<Info>) -> impl Responder {
    let Info { content, count, filename } = info.into_inner();
    match content_and_repeat(&content, count as usize, &filename) {
       Ok(_) => {
    let host = req.connection_info().host().to_string();
    let download_link = format!("http://{}/files/{}.docx", host, filename);
    let downloadable_link = format!("<a href='{}' download>Download file: <span class='text-sky-400/100'>{}.docx</span></a>", download_link, filename);
    HttpResponse::Ok().body(downloadable_link)
},
        Err(e) => HttpResponse::InternalServerError().body(format!("<div id='error-message'>Error: {}</div>", e)),
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> { 
    name: &'a str, 
}

#[get("/")]
async fn index() -> impl Responder {
    let tmpl = IndexTemplate { name: "Bindi" };
    HttpResponse::Ok().body(tmpl.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
    App::new()
    .service(index)
    .route("/generate", web::post().to(generate))
        .service(fs::Files::new("/static", ".").show_files_listing())
        .service(fs::Files::new("/files", "./files").show_files_listing())
})
.bind("0.0.0.0:8080")?
.run()
.await
}