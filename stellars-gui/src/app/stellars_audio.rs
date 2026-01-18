use std::sync::{Arc, RwLock};
use cpal::{ChannelCount, FromSample, Sample, SampleFormat, Stream, StreamError};
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
            let supported_config = configs.find(|c| c.sample_format() == SampleFormat::I16);

            if let Some(supported_config) = supported_config {
                let config = supported_config.with_max_sample_rate();
                let sample_rate = config.sample_rate();
                let channels = config.channels();

                println!("Configured audio device:");
                println!("\tSample Rate: {}Hz", sample_rate);
                println!("\tNumber of channels: {}", channels);

                libstellars.read().unwrap().use_audio(sample_rate as usize);
                let stellars = libstellars.clone();
                let stream = device.build_output_stream(
                    &config.config(),
                    move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                        audio_callback(data, stellars.clone(), channels);
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

fn audio_callback<T>(data: &mut [T], stellars: Arc<RwLock<Stellar>>, nb_channels: ChannelCount)
where T: Sample + FromSample<i16>
{
    let num_frames = data.len() / nb_channels as usize;
    let ch0_samples = stellars.read().unwrap().get_channel_0_samples(num_frames);
    let ch1_samples = stellars.read().unwrap().get_channel_1_samples(num_frames);

    for (frame_index, frame) in data.chunks_mut(nb_channels as usize).enumerate() {
        let ch0_i16 = (ch0_samples[frame_index] as i16 - 128) * 256;
        let ch1_i16 = (ch1_samples[frame_index] as i16 - 128) * 256;
        let mixed = ((ch0_i16 as i32 + ch1_i16 as i32) / 2) as i16;

        for sample in frame.iter_mut() {
            *sample = T::from_sample((mixed as f32 * 0.1) as i16);
        }
    }
}

fn audio_error(err: StreamError) {
    eprintln!("Audio error: {}", err);
}