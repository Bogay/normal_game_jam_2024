use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, StreamConfig};
use std::sync::mpsc::channel;
use std::time::Duration;
use vosk::{DecodingState, Model, Recognizer};

pub fn record_speech() {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("failed to get default input device");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config")
        .config();

    let model_path = "./vosk-model-small-en-us-0.15";
    let model = Model::new(model_path).unwrap();
    let mut recognizer = Recognizer::new(&model, config.sample_rate.0 as f32).unwrap();
    // recognizer.set_max_alternatives(10);
    // recognizer.set_words(true);
    // recognizer.set_partial_words(true);

    let (sender, receiver) = channel();

    let read_audio_handle = {
        let sender = sender.clone();
        std::thread::spawn(move || {
            let input_stream: cpal::Stream = device
                .build_input_stream(
                    &config,
                    move |data: &[i16], _info| {
                        let data = data.to_vec();
                        sender.send(data).expect("failed to send audio data");
                    },
                    |err| {
                        eprintln!("audio input err: {err}");
                    },
                    None,
                )
                .expect("failed to build input stream");
            input_stream.play().expect("failed to play input");

            loop {
                std::thread::sleep(Duration::from_secs(30));
            }
        })
    };

    let speech_to_text_handle = std::thread::spawn(move || loop {
        let sample = receiver.recv().expect("failed to recv audio data");
        if matches!(
            recognizer.accept_waveform(&sample),
            DecodingState::Finalized
        ) {
            let result = recognizer.final_result();
            println!("{result:?}");
        }
    });

    read_audio_handle.join().unwrap();
    speech_to_text_handle.join().unwrap();
}
