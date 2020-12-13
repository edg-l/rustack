## Rustack
An opinionated fullstack web template for rust that glues actix.rs with other components.

Work in progress.

# Usage
First clone:

```bash
git clone https://github.com/edg-l/rustack
```

Run the script `setup.sh` after cloning for an automated setup.

If you select yes to the option to create a database user and your psql "postgres" user is behind a password, you can provide it with `PGPASSWORD=password ./setup.sh`

You need `diesel_cli` tool installed.

## Tech used

- Database: [diesel.rs (postgresql for now)](https://diesel.rs/) ORM.
- Templates: [handlebars](https://handlebarsjs.com/).
- Css: [tailwindcss](https://tailwindcss.com/).
- Web framework: [actix.rs](https://actix.rs/).
- Validation: [validator](https://github.com/Keats/validator)
- SMTP: [lettre](https://github.com/lettre/lettre)
- Markdown: [comrak](https://github.com/kivikakk/comrak)
- Password encryption: [rust-argon2](https://github.com/sru-systems/rust-argon2)
- Cache: [cached](https://github.com/jaemk/cached)

# TODO
- Add option to generate a users table automatically + code.
- Add bin program to automate stuff afterwards.
