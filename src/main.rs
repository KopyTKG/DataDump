use actix_web::{web, App, HttpServer, HttpRequest , HttpResponse, Responder};
// use actix_multipart::Multipart;
use actix_files::NamedFile;
use image::{open, ImageFormat};
use std::io::Cursor;


async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}


async fn send_image(req: HttpRequest) -> impl Responder {
    let query = req.query_string();

    if query == "original" || query == ""{
        // Attempt to serve the original image directly
        match NamedFile::open("storage/testing.png") {
            Ok(file) => file.use_last_modified(true).use_etag(true).into_response(&req),
            Err(_) => HttpResponse::BadRequest().body("Error loading image")
        }
    } else {
        let query_pairs = serde_urlencoded::from_str::<Vec<(String, String)>>(query).unwrap_or_default();
        let (mut width, mut height) = (300, 300); // Default size
        for (key, value) in query_pairs {
            if key == "w" {
                width = value.parse::<u32>().unwrap_or(300);
            } else if key == "h" {
                height = value.parse::<u32>().unwrap_or(300);
            }
        }

        // Load and process the image
        let img = match open("storage/testing.png") {
            Ok(img) => img,
            Err(_) => return HttpResponse::BadRequest().body("Error loading image"),
        };

        // Resize the image using a high-quality filter
        let resized_img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

        // Convert the image to a byte vector
        let mut bytes: Vec<u8> = Vec::new();
        match resized_img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png) {
            Ok(_) => HttpResponse::Ok().content_type("image/png").body(bytes),
            Err(_) => HttpResponse::BadRequest().body("Error processing image")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/image", web::get().to(send_image))
    })
    .bind("0.0.0.0:10800")?
    .run()
    .await
}
