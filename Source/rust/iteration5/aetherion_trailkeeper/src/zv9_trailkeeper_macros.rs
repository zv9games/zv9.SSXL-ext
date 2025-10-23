
/// 🧠 Macro for logging structured events to Trailkeeper.
#[macro_export]
macro_rules! log_event {
    (
        event_type: $event_type:expr,
        actor: $actor:expr,
        description: $description:expr,
        affected_components: [$($component:expr),*],
        status: $status:expr
    ) => {{
        let entry = $crate::zv9_trailkeeper_entry::LogEntry {
            event_type: $event_type,
            timestamp: chrono::Utc::now(),
            actor: $actor.to_string(),
            description: $description.to_string(),
            affected_components: vec![$($component.to_string()),*],
            status: $status,
        };
        $crate::zv9_trailkeeper_collector::Trailkeeper::record(entry);
    }};

    (
        $event_type:expr,
        $actor:expr,
        $description:expr
    ) => {{
        let entry = $crate::zv9_trailkeeper_entry::LogEntry {
            event_type: $event_type,
            timestamp: chrono::Utc::now(),
            actor: $actor.to_string(),
            description: $description.to_string(),
            affected_components: vec![],
            status: $crate::zv9_trailkeeper_entry::LogStatus::Success,
        };
        $crate::zv9_trailkeeper_collector::Trailkeeper::record(entry);
    }};
}

/// 📘 Macro for registering API components in Trailkeeper.
#[macro_export]
macro_rules! log_component {
    ($name:expr, $desc:expr) => {
        $crate::zv9_trailkeeper_registry::register_component($name, $desc);
    };
}

#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
    use crate::zv9_prelude::*;
	use crate::trailkeeper::EventType;

    #[test]
    fn dummy_macro_usage() {
        log_event!(
            EventType::System,
            "macro_test",
            "Macro initialized for clean build"
        );

        log_component!("TestAPI", "Dummy API component for macro test");
    }
}


// the end