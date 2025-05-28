use owo_colors::OwoColorize;

pub mod info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub fn print_built_info() {
    let same_arch = info::TARGET == info::HOST;

    print!("{}", info::PKG_NAME.underline());
    print!(" {}", format!("v{}", info::PKG_VERSION).bold().green());
    print!(" built with {}", info::RUSTC_VERSION.bold().bright_red());
    print!(" in {} mode", info::PROFILE.bold().yellow());
    print!(" for {}", info::TARGET.bold().bright_blue());
    if !same_arch {
        print!(" on {}", info::HOST.bright_magenta().bold());
    }
    if let Some(platform) = info::CI_PLATFORM {
        if same_arch {
            print!(" on")
        }
        print!(" {}", platform.bright_magenta().bold());
    }
    print!(" at {}", info::BUILT_TIME_UTC.bright_cyan().bold());
    println!(".");
}
