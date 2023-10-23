<div align="center">
  <h1>mCaptcha PoW CLI</h1>
  <p>
    <strong>mCaptcha - PoW based DoS protection</strong>
  </p>

[![Documentation](https://img.shields.io/badge/docs-master-blue)](https://mcaptcha.github.io/mCaptcha/mcaptcha_cli/index.html)
![CI (Linux)](<https://github.com/mCaptcha/cli/workflows/CI%20(Linux)/badge.svg>)
[![dependency status](https://deps.rs/repo/github/mCaptcha/cli/status.svg)](https://deps.rs/repo/github/mCaptcha/cli)
<br />
[![codecov](https://codecov.io/gh/mCaptcha/cli/branch/master/graph/badge.svg)](https://codecov.io/gh/mCaptcha/cli)

</div>

## Usage

### Modes:

1. Offline: Computes PoW over given CAPTCHA parameters
2. Online: CLI alternative for people using browsers without JavaScript
   support

```bash
CLI tool to solve mCaptcha

Usage: mcaptcha-cli <COMMAND>

Commands:
  offline  Compute PoW with offline parameters
  online   Compute PoW by fetching parameters from  CAPTCHA URL
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Offline

Help menu:

```bash
Compute PoW with offline parameters

Usage: mcaptcha-cli offline --salt <SALT> --phrase <PHRASE> --difficulty-factor <DIFFICULTY_FACTOR>

Options:
  -s, --salt <SALT>                            Salt with which PoW should be computed
  -p, --phrase <PHRASE>                        Phrase over which PoW should be computed
  -d, --difficulty-factor <DIFFICULTY_FACTOR>  Difficulty Factor
  -h, --help                                   Print help
```

Example usage:

```bash
13:28 atm@lab cli ±|online|→ mcaptcha-cli offline -s $(rand 32) -p $(rand 32) -d 50000
difficulty: 50000
nonce: 90507
original phrase: f351f333d44b2c6b5bf7f033b065bbb8fb5e9dd153bd402e43ed04425f5a3859
result: 340276562956196291522979356090220150471
```

(where rand is
[this](https://github.com/realaravinth/dotfiles/blob/6fc6c87cc912e17488a35c0d3327ecf393221270/scripts/rand#L20)
script)

#### Online

Fetches CAPTCHA parameters from CAPTCHA URL, computes PoW, validates
against given mCaptcha server and returns authorization token

Help menu:

```bash
Compute PoW by fetching parameters from  CAPTCHA URL

Usage: mcaptcha-cli online --url <URL>

Options:
  -u, --url <URL>  URL of the CAPTCHA. Example:  https://example.org/widget?sitekey=foo
  -h, --help       Print help
```

Example usage:

```bash
13:32 atm@lab cli ±|online ✗|→ mcaptcha-cli online -u https://demo.mcaptcha.org/widget?sitekey=pHy0AktWyOKuxZDzFfoaewncWecCHo23
Authorization token: 3xleN26OctBuVu3X4t6CYyUjErhaxQvz
```

## Funding

### NLnet

<div align="center">
	<img
		height="150px"
		alt="NLnet NGIZero logo"
		src="./docs/third-party/NGIZero-green.hex.svg"
	/>
</div>

<br />

2023 development is funded through the [NGI0 Entrust
Fund](https://nlnet.nl/entrust), via [NLnet](https://nlnet.nl/). Please
see [here](https://nlnet.nl/project/mCaptcha/) for more details.
