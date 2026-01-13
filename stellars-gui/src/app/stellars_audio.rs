use std::sync::{Arc, RwLock};
use cpal::{FromSample, Sample, Stream, StreamError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use libstellars::Stellar;

pub struct StellarsAudio {
    stream: Option<Stream>,
}

impl StellarsAudio {
    pub fn new(libstellars: Arc<RwLock<Stellar>>) -> Self {
        let mut audio_stream: Option<Stream> = None;

        if  let Some(device) = cpal::default_host().default_output_device() &&
            let Ok(mut configs) = device.supported_output_configs() &&
            let Some(config) = configs.next()
        {
            libstellars.read().unwrap().use_audio(config.with_max_sample_rate().sample_rate() as usize);
            let stellars = libstellars.clone();
            let stream = device.build_output_stream(
                &config.with_max_sample_rate().config(),
                move |data: &mut [u8], _: &cpal::OutputCallbackInfo| {
                    audio_callback(data, stellars.clone());
                },
                audio_error,
                None).expect("Output stream cannot be created.");
            stream.play().unwrap();

            audio_stream = Some(stream);
        } else {
            println!("No audio output device available.");
        }

        Self {
            stream: audio_stream,
        }
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            stream.pause().unwrap();
        }
        self.stream = None;
    }
}

fn audio_callback<T>(data: &mut [T], stellars: Arc<RwLock<Stellar>>)
where T: Sample + FromSample<u8>
{
    let ch0_samples = stellars.read().unwrap().get_channel_0_samples(data.len());
    let ch1_samples = stellars.read().unwrap().get_channel_1_samples(data.len());

    for (sample_index, frame) in data.chunks_mut(1).enumerate() {
        for sample in frame.iter_mut() {
            *sample = T::from_sample(((ch0_samples[sample_index] | ch1_samples[sample_index]) as f64 * 0.5) as u8);
        }
    }
}

fn audio_error(err: StreamError) {
    eprintln!("Audio error: {}", err);
}