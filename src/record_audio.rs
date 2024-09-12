use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavWriter, WavSpec};

pub fn record_audio(file_path: &str, record_duration: u64) {
    println!("Starting audio recording...");
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device available");
    let config = device.default_input_config().unwrap();
    println!("Audio configuration: {:?}", config);

    let spec = WavSpec {
        channels: config.channels() as u16,
        sample_rate: config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    println!("Recording audio with specs: {:?}", spec);

    let writer = Arc::new(Mutex::new(Some(WavWriter::create(file_path, spec).unwrap())));
    let writer_clone = writer.clone();

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut writer_lock = writer_clone.lock().unwrap();
            if let Some(writer) = writer_lock.as_mut() {
                for &sample in data {
                    let amplitude = (sample * i16::MAX as f32) as i16;
                    writer.write_sample(amplitude).unwrap();
                }
            }
        },
        move |err| {
            eprintln!("An error occurred on the input audio stream: {}", err);
        },
        None
    ).unwrap();

    stream.play().unwrap();
    println!("Recording started. Recording for {} seconds...", record_duration);
    thread::sleep(Duration::from_secs(record_duration));
    drop(stream);
    println!("Recording stopped. Finalizing the WAV file...");

    let mut writer_lock = writer.lock().unwrap();
    if let Some(writer) = writer_lock.take() {
        writer.finalize().unwrap();
        println!("WAV file finalized successfully.");
    } else {
        println!("No writer found to finalize.");
    }
}