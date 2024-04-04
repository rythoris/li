# li - yet another bookmark manager

> [!WARNING]
> This software is still in alpha stage.

`li` is a simple command-line program to help you manage links and query them using tags or regular expression.

## Getting started

### Install the binary

Use [`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html) to install the program from git:
```
cargo install --git 'https://github.com/rythoris/li'
```

### Database

Before you start using the program you need to initialize the database, `li` uses the [Postgresql](https://www.postgresql.org/) database to store the links and query them.

If you don't already have a Postgresql database you can use the following command to deploy a Postgresql database container using docker. Replace `$VOLUME_PATH` with the location where you want to store the PostgreSQL data.

If you already have a working Postgresql database you can skip this step.

```bash
docker run -d \
    --name "postgresql" \
    -p "127.0.0.1:5432:5432" \
    -e "POSTGRES_DB=postgresql" \
    -e "POSTGRES_USER=postgresql" \
    -e "POSTGRES_PASSWORD=postgresql" \
    -v "$VOLUME_PATH:/var/lib/postgresql/data" \
    postgres
```

For convenience and better user experience you can define `LI_DATABASE_URL` and the application should work just fine otherwise you have to pass the `--database-url` flag every time you want to use the application.

```bash
export LI_DATABASE_URL="postgresql://username:password@127.0.0.1/database_name"
li initdb

# or

li --database-url "postgresql://username:password@127.0.0.1/database_name" initdb
```

### Add Your First Link

```bash
li add https://github.com/rythoris/li
```

This command will automatically request the index page and it tries to parse the title and the description and it will add the link to the database.

Assuming that this is your first link the ID will be `1`, you can use this ID to edit, remove, or open the links:
```bash
# add foo and bar tags to the link
li edit 1 -t foo -t bar
li edit 1 -t foo,bar

# open the link
li open 1

# remove the link from the database
li remove 1
```

### Query The Links

Probably one of the most used and interesting sub-commands is the `query` commands which you can use to filter the data. I highly recommend you to check out the help page of this command for more information but here is some example:

```bash
# search for 'something'
li query 'something'

# by default query will only search the title you can change this by specifying
# --filter flag to specify specific fields.
li query --filter title,desc 'something'

# perform case insensitive matching. By default, li is case sensitive.
li query --ignore-case 'something'

# search using regular expression
li query --regex 'a[bc]'

# filter by tags
li query --tags foo,bar
li query --tags foo --tags bar
```

## License

This project is licensed under [BSD-3-Clause](https://opensource.org/license/bsd-3-clause/). See [LICENSE](./LICENSE) for more details.
