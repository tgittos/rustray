use crate::core::render;
use crate::{ChunkBounds, assemble_chunks, image_height, raytrace_chunk};

pub struct Threaded {
    num_threads: usize,
}

impl Threaded {
    pub fn new() -> Self {
        let num_threads = num_cpus::get().max(1);
        Threaded { num_threads }
    }

    pub fn render(&self, render: &render::Render) -> Vec<u8> {
        // split the render into horizontal strips for each thread
        let height = image_height(render);
        let threads = self.num_threads.max(1);
        let strip_height = (height + threads as u32 - 1) / threads as u32;

        let mut chunks = Vec::with_capacity(threads);
        std::thread::scope(|scope| {
            let mut handles = Vec::with_capacity(threads);

            for i in 0..threads {
                let y_start = i as u32 * strip_height;
                if y_start >= height {
                    break;
                }
                let y_end = (y_start + strip_height).min(height);

                let bounds = ChunkBounds {
                    x_start: 0,
                    x_end: render.width,
                    y_start,
                    y_end,
                };

                handles.push(scope.spawn(move || {
                    let mut thread_rng = rand::rng();
                    raytrace_chunk(&mut thread_rng, render, bounds)
                }));
            }

            for handle in handles {
                chunks.push(handle.join().expect("thread panicked during render"));
            }
        });

        assemble_chunks(&chunks, render.width, height)
    }
}
