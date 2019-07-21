# String inspector
[![CircleCI](https://circleci.com/gh/MatMoore/string-inspector.svg?style=svg)](https://circleci.com/gh/MatMoore/string-inspector)

Command line utility to inspect unicode strings

## How to install
`cargo install string-inspector`

## Usage
`string-inspector hello world 💩`

Example output:
```
[utf-8]
bytes: 68 65 6c 6c 6f 20 77 6f 72 6c 64 20 f0 9f 92 a9
chars: h  e  l  l  o     w  o  r  l  d     1f4a9

hello world 💩
```

See `string-inspector -h` for detailed usage.

## Library usage
See [documentation](https://docs.rs/string-inspector/0.0.1/string_inspector/).

## Versioning
This project follows [Semantic Versioning](https://semver.org/).

## Contributing
All contributions are welcome, but issues tagged with either of the following are good places to start:
- [Good first issue](https://github.com/MatMoore/string-inspector/labels/good%20first%20issue)
- [Help wanted](https://github.com/MatMoore/string-inspector/labels/help%20wanted)

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## License
All code is free to use under the [MIT license](LICENSE)
