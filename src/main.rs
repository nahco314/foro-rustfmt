#![feature(rustc_private)]

use anyhow::Result;
use rustfmt_nightly::{Edition, Input, StyleEdition, Verbosity, Version};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

struct NullOptions;

impl rustfmt_nightly::CliOptions for NullOptions {
    fn apply_to(self, _: &mut rustfmt_nightly::Config) {
        unreachable!();
    }
    fn config_path(&self) -> Option<&Path> {
        unreachable!();
    }

    fn edition(&self) -> Option<Edition> {
        unreachable!();
    }

    fn style_edition(&self) -> Option<StyleEdition> {
        unreachable!();
    }

    fn version(&self) -> Option<Version> {
        unreachable!();
    }
}

fn main() -> Result<()> {
    let start_time = std::time::Instant::now();

    let target_path = PathBuf::from("/home/nahco314/RustroverProjects/onefmt-rustfmt/src/main.rs");

    let (mut config, _) =
        rustfmt_nightly::load_config::<NullOptions>(Some(&target_path.parent().unwrap()), None)?;

    let mut out: Vec<u8> = Vec::with_capacity(1024);
    config.set().emit_mode(rustfmt_nightly::EmitMode::Stdout);
    config.set().verbose(Verbosity::Quiet);
    let mut session = rustfmt_nightly::Session::new(config, Some(&mut out));
    let res = session.format(Input::File(target_path))?;

    drop(session);

    let formatted = String::from_utf8(out)?;
    println!("{}", formatted);

    let elapsed = start_time.elapsed();
    let elapsed_micros = elapsed.as_micros();
    println!("elapsed_micros: {}", elapsed_micros);

    Ok(())
}
