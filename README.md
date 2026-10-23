# edu-cargo-deny-layers-rs

The example workspace from the blog post
[your architecture diagram can't fail a build - cargo-deny can](https://d34dl0ck.me/your-architecture-diagram-cant-fail-a-build/index.html).

A toy online shop in four crates that stack on top of each other:

```
shop-app      starts the program and plugs the parts together
   │
shop-web      answers the HTTP requests from the browser
   │
shop-orders   the rules: is this your order? is it cancelled?
   │
shop-store    reads and writes the database
```

`shop-web` must never use `shop-store` directly. Cargo is fine with that shortcut,
[cargo-deny](https://github.com/EmbarkStudios/cargo-deny) is not, once `deny.toml` says so.

## Walk through it

Every chapter of the post is a tag. Check one out and run what the table says:

```bash
cargo install cargo-deny
git clone https://github.com/sassman/edu-cargo-deny-layers-rs
cd edu-cargo-deny-layers-rs
git checkout 01-the-shop
```

| tag | chapter | run | you see |
|---|---|---|---|
| `01-the-shop` | The shop | `cargo run -q` | `order 7` |
| `02-cycle` | What cargo already catches | `cargo check` | `error: cyclic package dependency` |
| `03-shortcut` | What cargo already catches | `cargo check` | nothing - the shortcut builds |
| `04-the-rule` | Teaching the build our rule | `cargo deny check bans` | `error[banned]`, `bans FAILED` |
| `05-rule-green` | Teaching the build our rule | `cargo deny check bans` | `bans ok` |
| `06-third-party` | Not only our own crates | `cargo deny check bans` | `postgres` is banned |
| `07-dev-dependency` | Oooops, the tests | `cargo deny check bans` | `(dev) shop-web`, `bans FAILED` |
| `08-test-double` | Oooops, the tests | `cargo deny check bans` | `bans ok` - the tests use `shop-store-in-mem` |
| `09-unused-wrapper` | Keeping the guest list true | `cargo deny check bans`, then `cargo deny check -D warnings bans` | a warning and `bans ok`, then `bans FAILED` |
| `10-ci` | Bonus Track | `just arch` | `bans ok`, and the same check in `.github/workflows/` |

`git diff 03-shortcut 04-the-rule` shows exactly what a chapter changed.

The tags `02`, `03`, `04`, `06`, `07` and `09` are broken on purpose - that is what the
chapter is about. `main` is the last chapter and green.

## License

[MIT](LICENSE)
