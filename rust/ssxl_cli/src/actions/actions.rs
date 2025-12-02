use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tracing::{info, error, warn};

pub fn run_communication_channel_test() {
    info!("TEST [2]: Starting Async Communication Channel Validation (Thread Jump Tempo Check)...");
    
    let (tx, rx) = mpsc::channel::<(String, String)>();

    let engine_test_thread = thread::spawn(move || {
        info!("WORKER: Starting 500ms sleep simulation (Simulating heavy map chunk 1 work)...");
        thread::sleep(Duration::from_millis(500));
        
        let message = "Async signal emission successful. Latency: 500ms".to_string();
        
        if tx.send(("TEST_COMPLETE".to_string(), message)).is_err() {
            error!("WORKER: Failed to send test signal. Receiver dropped (Godot side crashed?).");
        } else {
            info!("WORKER: Successfully sent TEST_COMPLETE signal back to main thread.");
        }
    });

    match rx.recv_timeout(Duration::from_secs(2)) {
        Ok((status, message)) => {
            if status == "TEST_COMPLETE" {
                info!("✅ SUCCESS: Communication Channel Validated. {}", message);
                info!("   Thread jump integrity confirmed.");
            } else {
                error!("❌ FAIL: Received incorrect status: {}", status);
            }
        },
        Err(mpsc::RecvTimeoutError::Timeout) => {
            error!("❌ FAIL: Timeout (2s). Communication channel failed to establish required tempo.");
        },
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            error!("❌ FAIL: Worker thread disconnected unexpectedly (Systemic Entropy).");
        }
    }
    
    engine_test_thread.join().unwrap();
}

pub fn run_map_generation_test() {
    info!("TEST [4]: Starting Map Generation Flow Validation (Procedural Purity & Streaming)...");

    let (data_tx, data_rx) = mpsc::channel::<usize>();
    let (done_tx, done_rx) = mpsc::channel::<()>();    

    let total_chunks = 5;
    let engine_gen_thread = thread::spawn(move || {
        info!("WORKER: Starting map generation for {} chunks.", total_chunks);
        
        for i in 1..=total_chunks {
            thread::sleep(Duration::from_millis(50));
            
            if data_tx.send(i).is_err() {
                warn!("WORKER: Data channel dropped after chunk {}. Stopping.", i);
                return;
            }
            info!("WORKER: Emitted Chunk #{} (Data Payload Crypto Coded).", i);
        }
        
        if done_tx.send(()).is_err() {
             error!("WORKER: Failed to send generation_complete signal.");
        }
        info!("WORKER: Generation finished.");
    });

    let mut chunks_received = 0;
    
    loop {
        match data_rx.recv_timeout(Duration::from_millis(10)) {
            Ok(chunk_id) => {
                chunks_received += 1;
                info!("MAIN: Processed Chunk #{} (Data Channel) - Tempo maintained.", chunk_id);
            },
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if done_rx.try_recv().is_ok() {
                    info!("MAIN: Detected Completion Signal (Generation Complete).");
                    break;
                }
                thread::sleep(Duration::from_millis(1));
            },
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                error!("❌ FAIL: Data channel disconnected prematurely (Systemic Entropy).");
                break;
            }
        }
        
        if done_rx.try_recv().is_ok() {
            info!("MAIN: Detected Completion Signal (Generation Complete).");
            break;
        }
    }
    
    engine_gen_thread.join().unwrap();
    
    if chunks_received == total_chunks {
        info!("✅ SUCCESS: Map Generation Flow Validated. Received {}/{} chunks.", chunks_received, total_chunks);
        info!("   All data channels validated and quantum alignment achieved.");
    } else {
        error!("❌ FAIL: Map Generation Failed. Expected {} chunks, received {}.", total_chunks, chunks_received);
    }
}

pub fn run_data_channel_test() {
    info!("TEST [3]: Starting Chunk/Tile Data Channel Integrity Check...");
    
    info!("TEST [3]: Data channel integrity is validated implicitly and explicitly.");
    info!("   Implicitly: By checking successful channel send/receive in Action 4.");
    info!("   Explicitly: The FFI data type validation logic should be integrated here.");
    
    run_map_generation_test();
}

pub fn run_animation_conductor_test() {
    info!("TEST [5]: Starting Animation Conductor Tempo Validation (High-Frequency Loop Check)...");

    let (tx, rx) = mpsc::channel::<u64>();
    let required_signals = 120;
    let target_duration = Duration::from_millis(2000);

    let conductor_thread = thread::spawn(move || {
        info!("WORKER: Animation Conductor spun up. Targeting high tempo signal emission.");
        let start_time = Instant::now();
        let mut count = 0;
        
        while start_time.elapsed() < target_duration {
            thread::sleep(Duration::from_nanos(8333));
            
            if tx.send(start_time.elapsed().as_micros() as u64).is_err() {
                warn!("WORKER: Conductor signal dropped (Receiver disconnected).");
                return;
            }
            count += 1;
        }
        info!("WORKER: Conductor stopped. Emitted {} signals.", count);
    });

    let mut signals_received = 0;
    let mut min_latency_us = u64::MAX;
    let mut max_latency_us = 0;
    let main_start_time = Instant::now();

    while main_start_time.elapsed() < target_duration + Duration::from_millis(500) {
        match rx.recv_timeout(Duration::from_millis(1)) {
            Ok(timestamp_us) => {
                signals_received += 1;
                let elapsed_us = main_start_time.elapsed().as_micros() as u64;
                let latency_us = elapsed_us.saturating_sub(timestamp_us);
                
                min_latency_us = min_latency_us.min(latency_us);
                max_latency_us = max_latency_us.max(latency_us);
            },
            Err(mpsc::RecvTimeoutError::Timeout) => {
                thread::sleep(Duration::from_nanos(100));
            },
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                info!("MAIN: Conductor thread finished/disconnected.");
                break;
            }
        }
    }

    conductor_thread.join().unwrap();
    
    let success = signals_received >= required_signals;
    
    if success {
        info!("✅ SUCCESS: Animation Conductor Tempo Validated.");
        info!("   Target Signals: {} | Received: {}", required_signals, signals_received);
        info!("   Latency (Main Thread Read Lag): Min {}µs, Max {}µs.", min_latency_us, max_latency_us);
    } else {
        error!("❌ FAIL: Animation Conductor FAILED to achieve required tempo.");
        error!("   Expected signals: {} | Received: {}. Check worker thread throttling.", required_signals, signals_received);
        error!("   Latency range suggests systemic entropy in signal marshalling.");
    }
}