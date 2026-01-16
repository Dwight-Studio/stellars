use std::sync::{Arc, RwLock};
use cpal::{FromSample, Sample, SampleFormat, Stream, StreamError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use libstellars::Stellar;

pub struct StellarsAudio {
    stream: Option<Stream>,
}

impl StellarsAudio {
    pub fn new(libstellars: Arc<RwLock<Stellar>>) -> Self {
        let mut audio_stream: Option<Stream> = None;

        if  let Some(device) = cpal::default_host().default_output_device() &&
            let Ok(mut configs) = device.supported_output_configs()
        {
            let supported_config = configs.find(|c| c.sample_format() == SampleFormat::I16 && c.channels() == 2);

            if let Some(supported_config) = supported_config {
                let config = supported_config.with_max_sample_rate();
                let sample_rate = config.sample_rate();

                println!("Configured audio device with sample rate: {}", sample_rate);

                libstellars.read().unwrap().use_audio(sample_rate as usize);
                let stellars = libstellars.clone();
                let stream = device.build_output_stream(
                    &config.config(),
                    move |data: &mut [u8], _: &cpal::OutputCallbackInfo| {
                        audio_callback(data, stellars.clone());
                    },
                    audio_error,
                    None).expect("Output stream cannot be created.");
                stream.play().unwrap();

                audio_stream = Some(stream);
            } else {
                println!("No supported output audio device found.");
            }
        } else {
            println!("No audio output device available.");
        }

        Self {
            stream: audio_stream,
        }
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }
}

fn audio_callback<T>(data: &mut [T], stellars: Arc<RwLock<Stellar>>)
where T: Sample + FromSample<i16>
{
    let ch0_samples = stellars.read().unwrap().get_channel_0_samples(data.len());
    let ch1_samples = stellars.read().unwrap().get_channel_1_samples(data.len());

    for (sample_index, frame) in data.chunks_mut(1).enumerate() {
        for sample in frame.iter_mut() {
            let ch0_sample: i16 = ((ch0_samples[sample_index] as u32 * 32767) / 127) as i16;
            let ch1_sample: i16 = ((ch1_samples[sample_index] as u32 * 32767) / 127) as i16;
            *sample = T::from_sample((((ch0_sample | ch1_sample) as f32) * 0.05) as i16);
        }
    }
}

fn audio_error(err: StreamError) {
    eprintln!("Audio error: {}", err);
}