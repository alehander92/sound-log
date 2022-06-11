use std::thread;
use std::time::Duration;
use systemstat::{System, Platform};
// use audio_generator::{Generator};
// use std::fs::File;
// use std::io::BufReader;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};
use psutil::process::pids;

#[derive(Debug)]
enum Kind {
    Cpu,
    Tasks
}

#[derive(Debug)]
enum ProgramError {
    CpuLoadError,
    TasksError
}

fn play_sound(frequency: f32, duration_in_millis: u64, speed: f32, volume: f32) {
    // copied and adapted from https://docs.rs/rodio/latest/rodio/
    // thanks to authors for the example

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let volume_to_be_played = volume.min(1.0);

    let source = SineWave::new(frequency).
                    take_duration(Duration::from_millis(duration_in_millis)).
                    speed(speed).
                    amplify(volume_to_be_played);
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();

    // end of rodio code
}

trait Tracker {
    fn track(&mut self) -> Result<(), ProgramError>;
}


struct CpuTracker {
}

// for ease: TODO more user machine-specific handling
const PID_COUNT_MAX_LIMIT: usize = 1_000;
const PID_COUNT_DIFFERENCE_LIMIT: usize = 10;

struct TasksTracker {
    last_pid_count: usize
}

impl CpuTracker {
    fn track(&mut self) -> Result<(), ProgramError> {
        // usage based on systemstat repo example
        // https://github.com/unrelentingtech/systemstat/blob/master/examples/info.rs
        println!("track cpu");
        let system = System::new();
        loop {
            match system.cpu_load_aggregate() {
                Ok(cpu) => {
                    thread::sleep(Duration::from_secs_f32(1.0));
                    let cpu_load = cpu.done().unwrap();
                    println!("cpu: {}", (cpu_load.user + cpu_load.system) * 100.0);
                    play_sound(490.0, 200, 1.0, (cpu_load.user + cpu_load.system));
                    // let mut g = audio_generator::Sin::new(441.0, 40_000 * (cpu_load.user + cpu_load.system));
                    // println!("{}", g.sample());
                    // println!("{}", g.sample());
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
        loop {
            let pids_result = pids();
            if let Ok(pids_vec) = pids_result {
                let pid_count = pids_vec.len();
                println!("tasks: {}", pid_count);

                if self.last_pid_count != 0 {
                    if self.last_pid_count > pid_count {
                        // decreasing or increasing depending on > or <
                        // play_sound(
                        //     400.0, 200, 1.0, 
                        //     (self.last_pid_count - pid_count) as f32 + 2.0  / PID_COUNT_DIFFERENCE_LIMIT as f32);
                        //     // PID_COUNT_MAX_LIMIT as f32);
                        play_sound(
                            350.0, 200, 1.0,
                            1.0 / PID_COUNT_DIFFERENCE_LIMIT as f32);
                    } else if self.last_pid_count < pid_count {
                        for i in self.last_pid_count .. pid_count {
                            play_sound(400.0, 200, 1.0, 0.5);
                        }
                        // play_sound(
                        //     400.0, 200, 1.0, 
                        //     (pid_count - self.last_pid_count) as f32 + 2.0  / PID_COUNT_DIFFERENCE_LIMIT as f32);
                    }
                }
                self.last_pid_count = pid_count;
                thread::sleep(Duration::from_secs_f32(1.0));
            } else {
                println!("error: for pids_result");
                return Err(ProgramError::TasksError);
            }
        }
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
                let mut tracker = TasksTracker { last_pid_count: 0 };
                tracker.track()
            }
        }
    }
}

// sound-events --cpu --tasks
// sound-log --cpu --tasks
fn main() -> Result<(), ProgramError> {
    let mut tracked_kinds = vec![];
    for arg in &std::env::args().collect::<Vec<String>>()[1..] {
        match arg.as_str() {
            "--cpu"  => {
                tracked_kinds.push(Kind::Cpu);
            },
            "--tasks" => {
                tracked_kinds.push(Kind::Tasks);
            },
            _ => {
                println!("unexpected: {}", &arg);
            }
        }
    }

    println!("{:?}", tracked_kinds);
    for kind in tracked_kinds {
        thread::spawn(move || { 
            println!("kind");
            let a = kind.track();
            println!("{:?}", a);
            thread::sleep(Duration::from_millis(10));
        });
    }
    thread::sleep(Duration::from_millis(1_000_000));
    Ok(())
}

// tasks ram, others?
// threads
// options : sound, duration; log only changes?
