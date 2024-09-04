## embed_rs

follower the [embed rus doc](https://docs.rust-embedded.org/book/intro/index.html) to learn the rust embedded programming.

## download the binary

build the binary and download it to the board.

```shell
cargo build | openocd -f openocd.cfg -c "program target/thumbv7em-none-eabi/debug/embed_rs verify reset exit"
```