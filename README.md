A fun nim CLI based on [nimlib](https://github.com/Tanja-4732/nimlib).

To build: 
```sh
git clone https://github.com/maxinebeckie/nimnimnim
cd nimnimnim
cargo build --release
```
To run:
```sh
cargo run --release -- --help
```
I like to create a fish alias.
```fish 
alias --save nim="/path/to/nimnimnim/target/release/nimnimnim"
```
You can calculate nim sums, eg.
```sh
nim -s "1 3 5 7"
```
returns `*0`. The default rules are (1) can take any amount from one pile, and (2) taking never splits a pile. These can be changed according to the `-c` option. The config file should contain a `NimRule` struct in RON format. See the [RON documentation](https://github.com/ron-rs/ron) and the [nimlib documentation](https://docs.rs/nimlib/0.1.1/nimlib/index.html) for more information. The config option is still pretty janky, it will be improved in the future. Example:
```sh
echo "NimRule(take: Any, split: Always)" > config.ron
nim -c config.ron -s "1 3 5 7"
```

