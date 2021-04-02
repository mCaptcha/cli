use clap::{App, Arg};
use pow_sha256::ConfigBuilder;

fn main() {
    let matches = App::new("mCaptcha PoW CLI")
        .version("0.1.0")
        .author("Aravinth Manivannan <realaravinth@batsense.net>")
        .about("Generates PoW for mCaptcha")
        .arg(
            Arg::with_name("salt")
                .short("-s")
                .long("--salt")
                .value_name("STRING")
                .help("Salt with which PoW should be computed")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("phrase")
                .short("-p")
                .long("--phrase")
                .value_name("STRING")
                .help("Phrase over which PoW should be computed")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("difficulty_factor")
                .short("-d")
                .long("--difficulty")
                .value_name("INTEGER")
                .help("Difficulty factor")
                .takes_value(true),
        )
        .get_matches();
    let phrase = matches.value_of("phrase").unwrap();
    let salt = matches.value_of("salt").unwrap();
    let difficulty_factor: u32 = matches
        .value_of("difficulty_factor")
        .unwrap()
        .parse()
        .expect("Please enter an integer for difficulty");

    let config = ConfigBuilder::default().salt(salt.into()).build().unwrap();
    let work = config.prove_work(&phrase, difficulty_factor).unwrap();
    println!("difficulty: {}", &difficulty_factor);
    println!("nonce: {}", &work.nonce);
    println!("original phrase: {}", &phrase);
    println!("result: {}", &work.result);
}
