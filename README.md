<p align="center">
    <img src="https://cdn.discordapp.com/attachments/766673591659659264/1159222698703060992/kisspng-application-programming-interface-cloud-computing-5ae66518b78b98.2448583215250486007518.png?ex=65303d44&is=651dc844&hm=d7829d47bf0d888cca641da11dbf0e4878d0b4bfdc54c80cd9744b0a1a778c1a&" width="150">
    <br>
    <br>
    <img src="https://img.shields.io/badge/language-rust-red.svg" alt="Language">
    <img src="https://img.shields.io/github/stars/ValuONE/rust-rest-api-template-mysql" alt="Stars">
    <img src="https://img.shields.io/github/forks/ValuONE/rust-rest-api-template-mysql" alt="Forks">
    <img src="https://img.shields.io/github/issues/ValuONE/rust-rest-api-template-mysql" alt="Issues">
</p>

<h3 align="center">Rust REST-API Template</h3>

  <p align="center">
    An easy to use template to start your REST-API
    <br/>
    <br/>
    <a href="https://github.com/ValuONE/rust-rest-api-template-mysql"><strong>Explore the docs »</strong></a>
    <br/>
    <br/>

## About the Project

This project is a REST-API written in Rust, using the Actix-Web and SQLX as
the database library. It's designed to demonstrate how to create a simple yet powerful REST-API with Rust.

## Features

- **Authentication**: Implemented bearer auth for API endpoint authentication.
- **Logging**: Complete file and console logging.
- **Error Handling**: Robust error handling for better API reliability.
- **CRUD example**: Implemented example model to showcase CRUD operation.

## Built With

- [Rust](https://www.rust-lang.org/) - A systems programming language.
- [Actix-Web](https://actix.rs/) - A powerful and flexible web framework for Rust.
- [SQLX](https://github.com/launchbadge/sqlx) - An asynchronous, pure Rust SQL library with compile-time checked
  queries.
- [MySQL](https://www.mysql.com/) - A popular open-source relational database management system.

## Use as template

1. **Prerequisites**:
    - Install Rust: If you don't have Rust installed, follow
      the [official installation guide](https://www.rust-lang.org/tools/install).

2. **Use template and create repository**:
    - Click on ``Use this template`` and create a new repository
    - In the new repository click on ``Code``, copy the https link
   ```bash
   git clone <https-link>
   cd <project-title>
   ```

3. **Open the folder with a terminal**
    - Make sure your database is running
    - Install the SQLX-CLI
   ```bash
   cargo install sqlx-cli
   ```
    - Create new database and apply migration
   ```bash
   sqlx database create --database-url mysql://root:@localhost:3306/<your_database_name>
   
   sqlx migrate run --database-url mysql://root:@localhost:3306/<your_database_name>
   ```

4. **Setup environment variables**
    - Set following environment variables
   ```
   "IP" (default: 127.0.0.1)
   "PORT" (default: 4000)
   "DB_URL" REQUIRED (example: mysql://root:@localhost:3306/<your_database_name>)
   ``` 

5. **Run application**
    - To run the application just enter follow command
   ```bash
   cargo run
   ```

6. **Test enpoints**
    - Now you can enter ``http://<IP>:<PORT>/todo`` to test the get all todo endpoints
    - If you want to try out every endpoint just use the postman collection provided under tests

## Author

- [valu](https://github.com/ValuONE)
