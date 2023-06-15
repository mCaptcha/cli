/*
 * mCaptcha - A proof of work based DoS protection system
 * Copyright © 2023 Aravinth Manivannan <realravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use clap::*;
use pow_sha256::ConfigBuilder;



/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Salt with which PoW should be computed
    #[arg(short, long)]
    salt: String,

    /// Phrase over which PoW should be computed
    #[arg(short, long)]
    phrase: String,


    /// Difficulty Factor
    #[arg(short, long)]
    difficulty_factor: u32,
}

fn main() {
   let matches = Args::parse();
    let phrase = matches.phrase;
    let salt = matches.salt;
    let difficulty_factor = matches.difficulty_factor;

    let config = ConfigBuilder::default().salt(salt.into()).build().unwrap();
    let work = config.prove_work(&phrase, difficulty_factor).unwrap();
    println!("difficulty: {}", &difficulty_factor);
    println!("nonce: {}", &work.nonce);
    println!("original phrase: {}", &phrase);
    println!("result: {}", &work.result);
}
