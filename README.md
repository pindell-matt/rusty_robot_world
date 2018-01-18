# Rusty Robot World
Inspired by the [Turing School Project - Robot World](https://github.com/turingschool-examples/robot-world)

[ assumes you have Rust and PostgreSQL installed ]

Setting up the database:
```
echo DATABASE_URL=postgres://username:password@localhost/robot_world_terraformed > .env
diesel migration run
```

Starting the App:
```
cargo run
```
