pub use profile_impl::MemoryProfilerPlugin;

#[cfg(feature = "memory-profile")]
mod profile_impl {
    use alloc_track::{AllocTrack, BacktraceMode};
    use bevy::prelude::*;
    use std::alloc::System;
    use std::time::Duration;

    #[global_allocator]
    static GLOBAL_ALLOC: AllocTrack<System> = AllocTrack::new(System, BacktraceMode::None);

    #[derive(Resource)]
    struct MemoryProfileTimer {
        t: Timer,
    }

    #[cfg(feature = "memory-profile")]
    fn memory_profile(time: Res<Time>, mut memory_profile_timer: ResMut<MemoryProfileTimer>) {
        memory_profile_timer.t.tick(time.delta());

        if memory_profile_timer.t.just_finished() {
            let report = alloc_track::thread_report();
            info!("THREADS\n{report}");
        }
    }

    pub struct MemoryProfilerPlugin;

    impl Plugin for MemoryProfilerPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(MemoryProfileTimer {
                t: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
            })
            .add_systems(Update, memory_profile);
        }
    }
}

#[cfg(not(feature = "memory-profile"))]
mod profile_impl {
    use bevy::prelude::*;
    pub struct MemoryProfilerPlugin;

    impl Plugin for MemoryProfilerPlugin {
        fn build(&self, app: &mut App) {
            // Do nothing
        }
    }
}
