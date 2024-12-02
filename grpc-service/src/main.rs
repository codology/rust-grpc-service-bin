use tonic::{transport::Server, Request, Response, Status};
use prometheus::{Encoder, TextEncoder, IntCounter};
use tonic::transport::Channel;
use service::file_service_server::{FileService, FileServiceServer};
use service::{FileRequest, FileResponse};
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod service {
    tonic::include_proto!("service");
}

// Prometheus metrics
lazy_static::lazy_static! {
    static ref FILE_PROCESSING_COUNTER: IntCounter = 
        prometheus::register_int_counter!("file_processing_counter", "Total number of processed files").unwrap();
}

#[derive(Default)]
pub struct MyFileService;

#[tonic::async_trait]
impl FileService for MyFileService {
    async fn process_file(&self, request: Request<FileRequest>) -> Result<Response<FileResponse>, Status> {
        // Process the file
        let data = request.into_inner().data;
        
        // Example processing: write to file
        let file_name = "output_file.bin";
        std::fs::write(file_name, &data).unwrap();

        // Increment Prometheus counter
        FILE_PROCESSING_COUNTER.inc();

        // Return status
        let response = FileResponse {
            status: "File processed successfully".to_string(),
        };
        Ok(Response::new(response))
    }
}

async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let file_service = MyFileService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(FileServiceServer::new(file_service))
        .serve(addr)
        .await?;

    Ok(())
}

async fn start_metrics_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9090";
    tokio::spawn(async move {
        warp::serve(warp::path!("metrics").map(move || {
            let mut buffer = Vec::new();
            let encoder = TextEncoder::new();
            let _ = encoder.encode(&prometheus::gather(), &mut buffer);
            warp::reply::with_header(buffer, "Content-Type", encoder.format_type())
        }))
        .run(([127, 0, 0, 1], 9090))
        .await;
    });
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start the gRPC server and Prometheus metrics server
    tokio::join!(
        start_grpc_server(),
        start_metrics_server(),
    );

    Ok(())
}
