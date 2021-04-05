## Rustack
An opinionated fullstack web template for rust that glues actix.rs with other components.

# Usage
First clone:

```bash
git clone https://github.com/edg-l/rustack
```

Run the script `setup.sh` after cloning for an automated setup.

Currently Rustack is preconfigured for postgresql, but with some changes you can make it work for any other database sqlx supports.

If you select yes to the option to create a database user and your psql "postgres" user is behind a password, you can provide it with `PGPASSWORD=password ./setup.sh`

You may want to install [sqlx-cli](https://crates.io/crates/sqlx-cli) to manage migrations.

The app configuration is under the `conf/` directory, currently everything else than `default.toml` is git ignored, you should not modify `default.toml` but rather create
`development.toml` on your development setup and `production.toml` on your production enviroment.

They will be merged after default.toml, this is all thanks to the [config](https://docs.rs/config/) crate.

Rustack also uses askama to render templates, I think it fits really well the idea of checking everything possible at compile time that rust itself follows.

It also has tailwindcss configured by default, you can easily remove it if you don't want it, note that there is a `build.rs` file that runs `yarn build` whenever any related file changes.

## Tech used

Note: Currently rustack uses beta releases of some crates: actix-web and its related crates, lettre.

- Database: [sqlx](https://github.com/launchbadge/sqlx).
- Templates: [askama](https://github.com/djc/askama).
- Css: [tailwindcss](https://tailwindcss.com/).
- Web framework: [actix.rs](https://actix.rs/).
- Validation: [validator](https://github.com/Keats/validator)
- SMTP: [lettre](https://github.com/lettre/lettre)
- Markdown: [comrak](https://github.com/kivikakk/comrak)
- Password encryption: [rust-argon2](https://github.com/sru-systems/rust-argon2)
- Cache: [cached](https://github.com/jaemk/cached)
- Sitemap: [sitewriter](https://github.com/edg-l/sitewriter)
