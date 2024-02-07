use actix_files::NamedFile;
use actix_web::{get, App, HttpResponse, HttpServer, Result};
use std::path::PathBuf;

#[get("/download")]
async fn download_file() -> Result<HttpResponse> {
    // Ruta del archivo que deseas descargar
    let file_path = PathBuf::from("archivo.txt");

    // Obtener el nombre del archivo para usarlo en la cabecera Content-Disposition
    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("archivo.txt");

    // Crear una respuesta con el archivo adjunto
    Ok(HttpResponse::Ok()
        .header("Content-Disposition", format!("attachment; filename=\"{}\"", file_name))
        .body(NamedFile::open(file_path)?))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(download_file)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
