use libcnb::data::build_plan::BuildPlan;
use libcnb::{
    cnb_runtime, BuildContext, DetectContext, DetectOutcome, GenericErrorHandler, GenericMetadata,
    GenericPlatform,
};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

fn main() {
    cnb_runtime(detect, build, GenericErrorHandler);
}

fn detect(
    context: DetectContext<GenericPlatform, GenericMetadata>,
) -> Result<DetectOutcome, libcnb::Error<StarterBuildpackError>> {
    println!(
        "Detect runs for application directory: {:?}",
        context.app_dir
    );

    Ok(DetectOutcome::Pass(BuildPlan::new()))
}

fn build(
    context: BuildContext<GenericPlatform, GenericMetadata>,
) -> Result<(), libcnb::Error<StarterBuildpackError>> {
    println!(
        "Build runs for application directory: {:?}",
        context.app_dir
    );

    Ok(())
}

#[derive(Debug)]
enum StarterBuildpackError {
    SomeError,
}

impl Display for StarterBuildpackError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StarterBuildpackError::SomeError => formatter.write_str("SomeError"),
        }
    }
}
