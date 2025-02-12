use axum::{
    extract::Path,
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_extra::TypedHeader;
use axum_extra::headers::Range;
use tokio::fs::File;
use axum_range::{KnownSize, Ranged};
use std::time::{SystemTime, UNIX_EPOCH};
use colored::*;
use memory_stats::memory_stats;

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

fn get_memory_usage() -> String {
    if let Some(usage) = memory_stats() {
        format!("Physical: {}, Virtual: {}", 
            format_bytes(usage.physical_mem as u64),
            format_bytes(usage.virtual_mem as u64)
        )
    } else {
        "Couldn't get memory stats".to_string()
    }
}

async fn stream_audio(
    Path(audio_id): Path<String>,
    range: Option<TypedHeader<Range>>,
) -> impl IntoResponse {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!("\n{} {} {}", 
        "🎵".bright_green(),
        format!("[{}]", timestamp).bright_blue(),
        format!("Streaming request started for audio ID: {}", audio_id).bright_yellow()
    );

    println!("{} {} {}", 
        "💾".bright_green(),
        "Initial memory usage:".bright_blue(),
        get_memory_usage().bright_magenta()
    );

    let file_path = format!("./cached_audio/{}.mp3", audio_id);
    let file = File::open(&file_path).await.expect("File not found");
    let metadata = file.metadata().await.unwrap();
    let file_size = metadata.len();
    
    println!("{} {} {}", 
        "📂".bright_green(),
        "File details:".bright_blue(),
        format!("Size: {}", format_bytes(file_size)).bright_magenta()
    );

    if let Some(ref range_header) = range {
        println!("{} {} {}", 
            "📦".bright_green(),
            "Range request:".bright_blue(),
            format!("{:?}", range_header).bright_cyan()
        );
    }

    let body = KnownSize::file(file).await.expect("Could not create body");
    
    println!("{} {} {}", 
        "💾".bright_green(),
        "Memory usage after setup:".bright_blue(),
        get_memory_usage().bright_magenta()
    );

    println!("{} {}", 
        "📊".bright_green(),
        "Streaming will be done in chunks (default 8KB) to minimize memory usage".bright_yellow()
    );

    let range = range.map(|TypedHeader(range)| range);
    Ranged::new(range, body)
}

#[tokio::main]
async fn main() {
    println!("\n{}", "=".repeat(70).bright_white());
    println!("{}", "🚀 Starting Audio Streaming Server with Memory Usage Monitoring".bright_green());
    println!("{}\n", "=".repeat(70).bright_white());

    println!("{} {} {}", 
        "💾".bright_green(),
        "Initial server memory usage:".bright_blue(),
        get_memory_usage().bright_magenta()
    );

    let app = Router::new().route("/audio/{id}", get(stream_audio));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("{} {}", 
        "🌍".bright_green(),
        format!("Server listening on {}", listener.local_addr().unwrap()).bright_yellow()
    );
    println!("{}", "Ready to stream audio files...".bright_green());
    println!("{}\n", "=".repeat(70).bright_white());

    axum::serve(listener, app).await.unwrap();
}
