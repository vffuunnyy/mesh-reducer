use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progess(iterations: u64) -> ProgressBar {
    let pb = ProgressBar::new(iterations);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    pb
}
