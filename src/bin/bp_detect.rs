use libcnb;
use libcnb::data::build_plan::BuildPlan;
use libcnb::detect::{DetectOutcome, GenericDetectContext};

use rust_cnb_starter::messages;

fn main() {
    libcnb::detect::cnb_runtime_detect(detect)
}

fn detect(_context: GenericDetectContext) -> Result<DetectOutcome, std::io::Error> {
    println!("{}", messages::DETECT_MESSAGE);
    Ok(DetectOutcome::Pass(BuildPlan::new()))
}
