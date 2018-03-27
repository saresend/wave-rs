use uuid::Uuid;
use cpal;
use std::thread;
use cpal::{StreamData, UnknownTypeOutputBuffer};
use std::sync::atomic::{AtomicBool, Ordering};
pub struct Node {
    id: Uuid,
    should_resign: AtomicBool,
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: Uuid::new_v4(),
            should_resign: AtomicBool::new(false),
        }
    }

    pub fn initialize(self) {
        let device = cpal::default_output_device().expect("Could not find device");
        let format = device
            .supported_output_formats()
            .unwrap()
            .next()
            .unwrap()
            .with_max_sample_rate();

        let event_loop_1 = cpal::EventLoop::new();
        let event_loop_2 = cpal::EventLoop::new();

        let stream_id = event_loop_1.build_output_stream(&device, &format).unwrap();
        event_loop_1.play_stream(stream_id);
        thread::spawn(move || 
        //This defines the output thread for the node
        event_loop_1.run(|stream_id, stream_data| {
            if self.should_resign.load(Ordering::Relaxed) {
                panic!("Closed Node");
                //TODO: Fix, this feels like a bad solution to cancel node execution
            }
            else {
                match stream_data {
                    StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                        for elem in buffer.iter_mut() {
                            *elem = u16::max_value() / 2;
                        }
                    },
                    StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                        for elem in buffer.iter_mut() {
                            *elem = 0;
                        }
                    },
                    StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                        for elem in buffer.iter_mut() {
                            *elem = 0.0;
                        }
                    },
                    _ => (),
                }
            }
        }));
    }
    pub fn resign(&mut self) {
        self.should_resign.store(true, Ordering::Relaxed)
    }
}
