## Rustack
Currently work in progress, it's not a template yet.

A fullstack web template for rust.

You can use this template using [cargo-generate](https://github.com/ashleygwilliams/cargo-generate), install it with:

`cargo install cargo-generate`

Then:

`cargo generate --git https://github.com/edg-l/rustack.git`

Once it's generated, you should copy the `.env.example` file to `.env` and edit it, then run `diesel setup` (you need diesel_cli installed).

You should also copy `default.toml` to `development.toml` under the `conf/` directory and configure it.

TODO: Automate this more.
