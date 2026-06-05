fn main() {
    println!("UAD Android Debloater");
    match run() {
        Ok(()) => println!("Done."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("loading debloat lists...");
    let (packages, had_errors) = uad_gui::core::uad_lists::load_debloat_lists(true);
    if had_errors {
        eprintln!("warning: some errors occurred while loading package lists");
    }
    match packages {
        Ok(pkgs) => eprintln!("loaded {} package definitions", pkgs.len()),
        Err(pkgs) => eprintln!("loaded {} package definitions (with fallback)", pkgs.len()),
    }

    Ok(())
}
