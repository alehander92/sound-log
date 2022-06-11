use std::thread;
use std::time::Duration;
use systemstat::{System, Platform};
// use audio_generator::{Generator};
// use std::fs::File;
// use std::io::BufReader;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};


#[derive(Debug)]
enum Kind {
    Cpu,
    Tasks
}

#[derive(Debug)]
enum ProgramError {
    CpuLoadError
}

trait Tracker {
    fn track(&mut self) -> Result<(), ProgramError>;
}

struct CpuTracker {
}

struct TasksTracker {
}

impl CpuTracker {
    fn track(&mut self) -> Result<(), ProgramError> {
        // usage based on systemstat repo example
        // https://github.com/unrelentingtech/systemstat/blob/master/examples/info.rs
        let system = System::new();
        loop {
            match system.cpu_load_aggregate() {
                Ok(cpu) => {
                    thread::sleep(Duration::from_secs(1));
                    let cpu_load = cpu.done().unwrap();
                    println!("cpu: {}", (cpu_load.user + cpu_load.system) * 100.0);
                    // let mut g = audio_generator::Sin::new(441.0, 40_000 * (cpu_load.user + cpu_load.system));
                    // println!("{}", g.sample());
                    // println!("{}", g.sample());

                    // copied from https://docs.rs/rodio/latest/rodio/
                    // thanks to authors for 
                    
                    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                    let sink = Sink::try_new(&stream_handle).unwrap();

                    // Add a dummy source of the sake of the example.
                    let source = SineWave::new(490.0).
                        take_duration(Duration::from_secs_f32(0.20)).
                        amplify(cpu_load.user + cpu_load.system);
                    sink.append(source);

                    // The sound plays in a separate thread. This call will block the current thread until the sink
                    // has finished playing all its queued sounds.
                    sink.sleep_until_end();

                    // end of rodio code
                },
                Err(e) => {
                    println!("error: {:?}", e);
                    return Err(ProgramError::CpuLoadError);
                }
            }
        }
    }
}

impl TasksTracker {
    fn track(&mut self) -> Result<(), ProgramError> {
        unimplemented!();
    }
}

impl Kind {
    fn track(&self) -> Result<(), ProgramError> {
        match self {
            Kind::Cpu => {
                let mut tracker = CpuTracker {};
                tracker.track()
            },
            Kind::Tasks => {
                let mut tracker = TasksTracker {};
                tracker.track()
            }
        }
    }
}

// sound-events --cpu --tasks
// sound-log --cpu --tasks
fn main() -> Result<(), ProgramError> {
    let mut tracked_kinds = vec![];
    for arg in std::env::args() {
        match arg.as_str() {
            "--cpu"  => {
                tracked_kinds.push(Kind::Cpu);
            },
            "--tasks" => {
                tracked_kinds.push(Kind::Tasks);
            },
            _ => {
            }
        }
    }

    for kind in tracked_kinds {
        kind.track()?;
    }
    Ok(())
}
