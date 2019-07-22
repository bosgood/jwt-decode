# jwt-decode

Decodes JSON web tokens for debugging purposes.

## Usage

```bash
$ jwt-decode eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
{"alg":"HS256","typ":"JWT"} {"iat":1516239022,"name":"John Doe","sub":"1234567890"}
```

## Installation

1. Download the latest version from the [Releases page](https://github.com/bosgood/jwt-decode/releases)
2. Unzip the archive and place `jwt-decode` in your `PATH`.

## Building from source

```
$ cargo build
```

## Running the unit tests

```
$ cargo test
```

## License

[MIT License](https://github.com/bosgood/structy/blob/master/LICENSE). This software is provided as-is, without warranty of any kind.
