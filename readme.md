# Rust Todo cli app

## Implements

- `sqlite` using [`rusqlite`](https://crates.io/crates/rusqlite) crate

## Usage

`todo <command> <args>`

### commands

| command | description                                            | example             |
| ------- | ------------------------------------------------------ | ------------------- |
| [null]  | list current todos                                     | todo                |
| help    | Display help menu                                      | todo help           |
| add     | Add a todo                                             | todo add do laundry |
| done    | Complete todos by index. (get index by running `todo`) | todo done 1 5       |

## run app

### cargo

```sh
cargo run help
```

### docker

```sh
# build image
docker build -t aidanwallace/rust-todo .

# run image
docker run --rm -v ./data:/data aidanwallace/rust-todo help
```
