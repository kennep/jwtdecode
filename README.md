# jwtdecode

This is a simple JWT decoder written in Rust.

## Download

Pre-built binaries can be found under [Releases](https://github.com/kennep/jwtdecode/releases).
Just download the correct executable for your operating system and put it somewhere on your
`PATH` to use it.

## Building

You can also build your own executable:

```bash
$ cargo build
```

## Usage

This program reads a JWT either from standard input or from a file, and outputs
the decoded header and payload, as well as the signature, to standard output.

The filename to read the JWT from is the first and only argument.
The default, if no filename is given, is to read the JWT from standard input.

Example

```bash
$ echo "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.
> eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.
> SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c" | jwtdecode
{
  "header": {
    "alg": "HS256",
    "typ": "JWT"
  },
  "payload": {
    "iat": 1516239022,
    "name": "John Doe",
    "sub": "1234567890"
  },
  "signature": "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
}
```

You can also extract a field by JSONPATH:

```bash
$ echo "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.
> eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.
> SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c" | jwtdecode -p '$.header.alg'
"HS256"
```

## Feedback

Feedback is most welcome. Submit an issue or PR for this repository on GitHub.
