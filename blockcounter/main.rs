extern crate blockcounter;
#[macro_use]
extern crate clap;

#[cfg(not(windows))]
extern crate isatty;

#[cfg(not(windows))]
#[inline(always)]
fn stdin_isatty() -> bool {
    isatty::stdin_isatty()
}

#[cfg(windows)]
#[inline(always)]
fn stdin_isatty() -> bool {
    true
}

fn main() {
    let is_a_tty = stdin_isatty();
    let yaml = if is_a_tty {
        load_yaml!("en_base.yml").clone()
    } else {
        load_yaml!("en.yml").clone()
    };
    let args = clap::App::from_yaml(&yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    let tolerance = args.value_of("TOLERANCE");
    let tolerance = match tolerance {
        Some(t) => t.parse().unwrap(),
        None => 2,
    };

    if is_a_tty {
        let file = args.value_of("DATA").unwrap().to_string();
        let file = std::fs::File::open(&file).unwrap();
        let file = std::io::BufReader::new(file);
        println!("{}", blockcounter::count_blocks(tolerance, file));
    } else {
        use std::io::Read;
        let stdin_ = std::io::stdin();
        let mut s = String::new();
        let _ = stdin_.lock().read_to_string(&mut s).unwrap();
        println!("{}", blockcounter::count_blocks(tolerance, s.as_bytes()));
    }
}
