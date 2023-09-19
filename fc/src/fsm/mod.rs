use std::time::Duration;

pub mod states;

#[non_exhaustive]
#[derive(Debug)]
pub(crate) enum Event {
    Arm,
    LiftoffDetected,
    WeightlessnessDetected,
    ApogeeDetected,
    SeparationTriggered,
    SeparationDetected,
    MainDeploymentAltitudeDetected,
    MainDeploymentComplete,
    PrebrakeTriggered,
    PrebrakeComplete,
    ControlLoopTick(Duration),
    Landed,
    SystemShutdownTriggered
}
