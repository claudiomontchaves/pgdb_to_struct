# pgdb_to_struct

This is a Rust CLI application to generate Rust struct files from Postgres database tables.

Let's assume we have a Postgres database with the following tables:

```sql
CREATE TABLE public.customer (
	id serial4 NOT NULL,
	first_name varchar(40) NOT NULL,
	last_name varchar(40) NOT NULL,
	city varchar(40) NULL,
	country varchar(40) NULL,
	phone varchar(20) NULL,
	CONSTRAINT pk_customer PRIMARY KEY (id)
);

CREATE TABLE public.product (
	id serial4 NOT NULL,
	product_name varchar(50) NOT NULL,
	supplier_id int4 NOT NULL,
	unit_price numeric(12, 2) NULL DEFAULT 0,
	package varchar(30) NULL,
	is_discontinued bool NOT NULL DEFAULT false,
	CONSTRAINT pk_product PRIMARY KEY (id)
);

```

To generate Rust structs that represent these tables, first edit the file **app.properties** to set the database connection properties and the table names to be used.

```yaml
db_host: 127.0.0.1
db_port: 5432
db_name: sample_db
db_user: postgres
db_password: sample@123
db_schema: public
tables: customer, product
use_serde: false
```

Then build and run using **cargo**:

```shell
$ cargo build
```

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/pgdb_to_struct`
>> Creating struct for table: customer
>> Creating struct for table: product
```

A new folder named **gen** will be created with two Rust source files:

```shell
gen/
   customer.rs
   product.rs
```

Here is the content of the generated files:

### customer.rs

```rust
#[derive(Debug)]
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub country: String,
    pub phone: String,
}
```

### product.rs

```rust
#[derive(Debug)]
pub struct Product {
    pub id: i32,
    pub product_name: String,
    pub supplier_id: i32,
    pub unit_price: bigdecimal::BigDecimal,
    pub package: String,
    pub is_discontinued: bool,
}
```

Optionally you can include **serde** lib for serialization/deserialization. In **app.properties** make **use_serde: true** and you will get:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub first_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub last_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub city: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub country: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub phone: String,
}
```
