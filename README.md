# Rusty Robot World
Inspired by the [Turing School Project - Robot World](https://github.com/turingschool-examples/robot-world)

[ assumes you have Rust and PostgreSQL installed ]

Setting up the database:
```
cargo install diesel_cli
echo DATABASE_URL=postgres://username:password@localhost/rusty_robot_world > .env
diesel migration run
```

Starting the App:
```
cargo run
```
