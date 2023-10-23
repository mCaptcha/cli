// Copyright © 2023 Aravinth Manivannan <realravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::time::Instant;

use clap::*;
use pow_sha256::ConfigBuilder;
use pow_sha256::PoW;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Parser, Serialize, Clone, Debug)]
/// Compute PoW with offline parameters
struct Offline {
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

#[derive(Deserialize, Parser, Serialize, Clone, Debug)]
/// Compute PoW by fetching parameters from  CAPTCHA URL
struct Online {
    /// URL of the CAPTCHA. Example:  https://example.org/widget?sitekey=foo
    #[arg(short, long)]
    url: Url,
}

impl Online {
    fn extract_sitekey(&self) -> Option<String> {
        let mut sitekey = None;
        for (k, v) in self.url.query_pairs() {
            if &k == "sitekey" {
                let x: &str = &v;
                sitekey = Some(x.to_string())
            }
        }

        sitekey
    }

    fn get_config_url(&self) -> Url {
        let mut url = self.url.clone();
        url.set_path("/api/v1/pow/config");
        url
    }

    fn get_verify_url(&self) -> Url {
        let mut url = self.url.clone();
        url.set_path("/api/v1/pow/verify");
        url
    }
}

#[derive(Deserialize, Parser, Serialize, Clone, Debug)]
#[command(author, version, about, long_about = None)]
enum Args {
    Offline(Offline),
    Online(Online),
}

fn prove_work(phrase: String, salt: String, difficulty_factor: u32) -> PoW<String> {
    let config = ConfigBuilder::default().salt(salt).build().unwrap();
    config.prove_work(&phrase, difficulty_factor).unwrap()
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args {
        Args::Offline(matches) => {
            let phrase = matches.phrase;
            let salt = matches.salt;
            let difficulty_factor = matches.difficulty_factor;
            let work = prove_work(phrase.clone(), salt, difficulty_factor);

            //            let config = ConfigBuilder::default().salt(salt.into()).build().unwrap();
            //            let work = config.prove_work(&phrase, difficulty_factor).unwrap();
            println!("difficulty: {}", &difficulty_factor);
            println!("nonce: {}", &work.nonce);
            println!("original phrase: {}", &phrase);
            println!("result: {}", &work.result);
        }
        Args::Online(matches) => {
            let sitekey = matches.extract_sitekey();

            if sitekey.is_none() {
                println!("ERROR: Sitekey not found in URL. Please enter correct URL");
                return;
            }
            let sitekey = sitekey.unwrap();

            let c = Client::default();
            let url = matches.get_config_url();

            #[derive(Clone, Debug, Serialize, Deserialize)]
            struct ConfigRequest {
                key: String,
            }

            #[derive(Clone, Debug, Serialize, Deserialize)]
            struct ConfigResp {
                string: String,
                difficulty_factor: u32,
                salt: String,
            }
            let req = ConfigRequest {
                key: sitekey.to_string(),
            };

            let resp: ConfigResp = c
                .post(url)
                .json(&req)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            let start = Instant::now();
            let work = prove_work(resp.string.clone(), resp.salt, resp.difficulty_factor);
            let finish = Instant::now();
            let time_elapsed = finish.duration_since(start);
            let time = time_elapsed.as_millis() as usize;

            #[derive(Clone, Debug, Serialize, Deserialize)]
            struct VerifyRequest {
                key: String,
                nonce: u64,
                result: String,
                string: String,
                time: usize,
                worker_type: String,
            }

            #[derive(Clone, Debug, Serialize, Deserialize)]
            struct VerifyResp {
                token: String,
            }

            let req = VerifyRequest {
                key: sitekey.clone(),
                nonce: work.nonce,
                result: work.result,
                string: resp.string,
                time,
                worker_type: String::from("mCaptcha CLI"),
            };
            let url = matches.get_verify_url();
            let resp: VerifyResp = c
                .post(url)
                .json(&req)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            println!("Authorization token: {}", resp.token);
        }
    }
}
