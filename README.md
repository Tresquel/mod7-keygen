# mod7 Keygen

A command line tool to generate CD, Office and OEM keys that use the mod7 algorithm

I used [this blog post](https://gurney.dev/posts/mod7/) to figure out how these keys work and how to generate them

# Usage
`mod7-keygen { --cd | --office | --oem } <integer>` or `mod7-keygen { -c | -e | -o } <integer>`
- The integer is optional, it specifies the amount of keys to generate.

`mod7-keygen --check <string>`
- Checks if the provided key is valid.

`mod7-keygen { --version | -v }`
- Prints the version of the program

# Building
Make sure you have Rust and Git installed
- `git clone https://github.com/Tresquel/mod7-keygen.git`
- `cd mod7-keygen && cargo build --release`