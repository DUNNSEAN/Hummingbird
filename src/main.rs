pub mod record_audio;

use std::{fs::File, io::Read};

use record_audio::record_audio;
use mutter::{Model, ModelType};

#[tokio::main]
async fn main() {
    let file_path = "recorded_audio.wav";
    // let record_duration = 5;  // Record for 5 seconds
    // record_audio(file_path, record_duration);
    
    // Initialize the model handler with the 'BaseEn' model
    println!("Initializing model handler...");
    let model = Model::download(&ModelType::BaseEn).expect("Failed to download model");
    println!("Model handler initialized.");
    
    // Read the WAV file into a Vec<u8>
    let mut file = File::open(file_path).expect("Failed to open audio file");
    let mut audio_data = Vec::new();
    file.read_to_end(&mut audio_data).expect("Failed to read audio file");

    // Create a new transcriber instance using the model handler
    println!("Transcribing audio file: {}", file_path);
    let transcription = model.transcribe_audio(audio_data, false, false, None)
        .expect("Failed to transcribe audio");
    println!("Transcription completed.");
    
    println!("Transcription: {}", transcription.as_text());
    println!("SRT: {}", transcription.as_srt());
}
