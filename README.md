# 🎵 Axum Audio Streamer PoC

A demonstration of memory-efficient audio streaming using Rust and Axum. This project shows how to implement HTTP byte-range requests for audio streaming while maintaining minimal memory footprint.

## 🚀 Features

- Efficient audio streaming with minimal memory usage
- Support for HTTP byte-range requests
- Real-time memory usage monitoring
- Handles multiple concurrent connections
- Supports seeking/scrubbing in audio players

## 📊 Memory Usage Demonstration

- Server baseline memory: ~10MB
- Streaming 50MB+ audio files uses only ~8KB chunks
- Multiple concurrent streams with minimal memory overhead
- Memory efficient even with frequent seeking/scrubbing

## 🛠 Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/axum-audio-streamer.git
cd axum-audio-streamer
```

2. Create a `cached_audio` directory and add some MP3 files:
```bash
mkdir cached_audio
cp your-audio-file.mp3 cached_audio/test.mp3
```

3. Build and run:
```bash
cargo run
```

## 📝 Usage

1. Start the server:
```bash
cargo run
```

2. Access your audio file:
```
http://localhost:8080/audio/test
```
(Replace 'test' with your audio file name without the .mp3 extension)

## 🔧 Technical Details

- Built with Axum 0.8
- Uses tokio for async I/O
- Implements HTTP byte-range requests
- Streams files in 8KB chunks
- Real-time memory usage monitoring

## 📈 Memory Usage Pattern

The server maintains a baseline memory footprint (~10-12MB) which includes:
- TCP listener
- Connection pools
- Runtime components
- HTTP server structures

Additional connections only add minimal overhead, as demonstrated by memory usage stats in the terminal output.

## 🧪 Testing

To verify the streaming behavior:
1. Monitor the terminal output for memory usage
2. Open multiple browser windows playing the same file
3. Try seeking/scrubbing through the audio
4. Notice that memory usage stays well below the actual file size

## 📦 Dependencies

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1.0", features = ["full"] }
axum-extra = { version = "0.10", features = ["typed-header"] }
axum-range = "0.5.0"
colored = "2.0"
memory-stats = "1.0"
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎥 Demo

[[Video](https://x.com/Lukaesch/status/1889780934540460400)]
