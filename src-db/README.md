# app-db

Sea-ORM declarations.

## To Generate

Install sea-orm-cli.

```sh
cargo install sea-orm-cli
```

Run it

```sh
DATABASE_URL=protocol://username:password@localhost/database
sea-orm-cli.exe generate entity -l -o src --with-serde serialize -u $DATABASE_URL
```
