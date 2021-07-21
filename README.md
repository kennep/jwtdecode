# jwtdecode

This is a simple JWT decoder written in Rust.

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
Header: {
  "alg": "HS256",
  "typ": "JWT"
}
Payload: {
  "iat": 1516239022,
  "name": "John Doe",
  "sub": "1234567890"
}
Signature: SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
```

## Building

```bash
$ cargo build
```

## Feedback

Feedback is most welcome. Submit an issue or PR for this repository on GitHub.


