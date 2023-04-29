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
The only thing currently working is calculating nim sums according to the rules (1) no pile splits and (2) can take as many matches from one pile as desired. 
```sh
nim -s "1 3 5 7"
```
returns `*0`. 


