use portaudio as pa;
use std::sync::mpsc::channel;
use std::time::Duration;
use vosk::{DecodingState, Model, Recognizer};

const SAMPLE_RATE: f64 = 16000.;
const FRAMES: u32 = 8192;

pub fn record_speech() {
    let pa = pa::PortAudio::new().unwrap();
    let def_input = pa.default_input_device().unwrap();
    let input_info = pa.device_info(def_input).unwrap();
    let latency = input_info.default_low_input_latency;
    let input_params = pa::StreamParameters::new(def_input, 1, true, latency);
    pa.is_input_format_supported(input_params, SAMPLE_RATE)
        .unwrap();
    let settings = pa::InputStreamSettings::new(input_params, SAMPLE_RATE, FRAMES);

    let model_path = "./vosk-model-small-en-us-0.15";
    let model = Model::new(model_path).unwrap();
    let mut recognizer = Recognizer::new(&model, SAMPLE_RATE as f32).unwrap();
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    let (sender, receiver) = channel();

    // A callback to pass to the non-blocking stream.
    let callback = move |pa::InputStreamCallbackArgs { buffer, frames, .. }| {
        assert!(frames == FRAMES as usize);
        sender.send(buffer.to_vec()).unwrap();
        pa::Continue
    };

    let read_audio_handle = {
        // let sender = sender.clone();
        std::thread::spawn(move || {
            let mut stream = pa.open_non_blocking_stream(settings, callback).unwrap();
            stream.start().unwrap();
            while stream.is_active().unwrap() {
                std::thread::sleep(Duration::from_secs(30));
            }
            stream.stop().unwrap();
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
