use binaryninja::binaryview::{BinaryView, BinaryViewBase, BinaryViewExt};
use binaryninja::disassembly::{DisassemblyOption, DisassemblySettings};
use binaryninja::function::Function;
use binaryninja::linearview::{LinearViewCursor, LinearViewObject};

use clap::Parser;

/// Use binaryninja to decompile to C.
#[derive(Parser, Debug)]
#[clap(version, long_about = None)]
struct Args {
    /// Decompile from a BNDB
    #[clap(long, short)]
    bndb: bool,

    /// Path to the file to decompile
    filename: String,
}

fn decompile_to_c(view: &BinaryView, func: &Function) -> String {
    let mut settings = DisassemblySettings::new();
    settings.set_option(DisassemblyOption::ShowAddress, false);
    settings.set_option(DisassemblyOption::WaitForIL, true);

    let linearview = LinearViewObject::language_representation(view, &settings);

    let mut cursor = LinearViewCursor::new(&linearview);
    cursor.seek_to_address(func.highest_address());

    let lines = view
        .get_previous_linear_disassembly_lines(&mut cursor.duplicate())
        .chain(view.get_next_linear_disassembly_lines(&mut cursor));

    for line in lines {
        println!("{}", line);
    }

    "".into()
}

fn main() {
    let args = Args::parse();

    eprintln!("Loading plugins...");
    binaryninja::headless::init();

    eprintln!("Loading binary...");
    let bv = binaryninja::open_view(args.filename).expect("Couldn't open `/bin/cat`");

    eprintln!("Filename:  `{}`", bv.metadata().filename());
    eprintln!("File size: `{:#x}`", bv.len());
    eprintln!("Function count: {}", bv.functions().len());

    for func in &bv.functions() {
        decompile_to_c(bv.as_ref(), func.as_ref());
    }

    binaryninja::headless::shutdown();
}
