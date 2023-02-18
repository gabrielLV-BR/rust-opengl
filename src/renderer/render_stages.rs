use bevy_ecs::schedule::StageLabel;

#[derive(StageLabel)]
pub(crate) struct InitStage;

#[derive(StageLabel)]
pub(crate) struct PrepareStage;

#[derive(StageLabel)]
pub(crate) struct RenderStage;

#[derive(StageLabel)]
pub(crate) struct CleanupStage;

#[derive(StageLabel)]
pub(crate) struct DisposeStage;