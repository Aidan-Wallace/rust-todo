# Rust Todo cli app

![GitHub All Releases][badge-1] ![Crates.io][badge-2] ![Docker Pulls][badge-3]

A simple rust cli tool for managing todos

## Implements

- `sqlite` using [`rusqlite`](https://crates.io/crates/rusqlite) crate

## Usage

`rust-todos <command> <args>`

### commands

| command | description                                                  | example                   |
| ------- | ------------------------------------------------------------ | ------------------------- |
| [null]  | list current todos                                           | rust-todos                |
| help    | Display help menu                                            | rust-todos help           |
| add     | Add a todo                                                   | rust-todos add do laundry |
| done    | Complete todos by index. (get index by running `rust-todos`) | rust-todos done 1 5       |
| clear   | Clear all todos                                              | rust-todos clear          |

## run app

### cargo

```sh
cargo run help
```

### docker

```sh
# build image
docker build -t aidanwallace/rust-todos .

# run image
docker run --rm -it -v ./data:/data aidanwallace/rust-todos help
```

[badge-1]: https://img.shields.io/github/downloads/aidan-wallace/rust-todos/total?label=GitHub%20Downloads
[badge-2]: https://img.shields.io/crates/d/rust-todos?label=Cargo%20Downloads
[badge-3]: https://img.shields.io/docker/pulls/aidanwallace/rust-todos
