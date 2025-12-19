use flume::{Receiver, Sender};
use crate::animate_events::AnimationEvent;
use crate::animate_worker::AnimationWorker;
use crate::shared_config::AnimationConfig;

#[derive(Debug)]
pub enum ControlMessage {
    Pause,
    Stop,
    UpdateConfig(AnimationConfig),
}

pub struct AnimationConductor {
    pub event_receiver: Receiver<AnimationEvent>,
    control_senders: Vec<Sender<ControlMessage>>,
    workers: Vec<AnimationWorker>,
}

impl AnimationConductor {
    pub fn new(config: &AnimationConfig) -> Self {
        let mut num_workers = config.worker_count;

        if num_workers == 0 {
            eprintln!("WARNING: Animation worker count is 0. Forcing to 1.");
            num_workers = 1;
        }

        let (event_sender, event_receiver) = flume::unbounded();

        let mut workers = Vec::with_capacity(num_workers);
        let mut control_senders = Vec::with_capacity(num_workers);

        for id in 0..num_workers {
            let (control_sender, control_receiver) = flume::unbounded();

            let worker = AnimationWorker::new(
                id,
                event_sender.clone(),
                control_receiver,
                config.clone(),
            );

            workers.push(worker);
            control_senders.push(control_sender);
        }

        eprintln!(
            "INFO: Animation Conductor: Launched {} dedicated animation workers.",
            num_workers
        );

        Self {
            event_receiver,
            control_senders,
            workers,
        }
    }

    pub fn shutdown(self) {
        eprintln!("INFO: Animation Conductor: Initiating graceful shutdown of workers.");

        for sender in self.control_senders.iter() {
            let _ = sender.send(ControlMessage::Stop);
        }

        for worker in self.workers {
            worker.join();
        }

        eprintln!("INFO: Animation Conductor: All animation workers shut down.");
    }

    pub fn get_event_receiver(&self) -> &Receiver<AnimationEvent> {
        &self.event_receiver
    }

    pub fn pause_workers(&self) {
        for sender in self.control_senders.iter() {
            let _ = sender.try_send(ControlMessage::Pause);
        }
    }
}
