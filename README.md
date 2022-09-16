# Baby Turkey

## *In-memory database made for a wide variety of use cases*

<div>&nbsp;</div>

## **Introduction**
 Baby Turkey is an optimized, robust, and easy to use in-memory document database with graph support.
 The database is written in both Python and Rust, and is optimized for speed, memory usage, and ease of access for object oriented programming. This project was inspired by [Dagoba](https://github.com/skvrahul/dagoba).

<div>&nbsp;</div>

## **Features**
- :watch: Asynchronous database operations
- :chart: Graph support, through both the use of native [Gobble Query Language]() along with Python and Rust bindings
- :bread: Rust, Python query bindings along with SQL statement support
- :cherries: Stored procedures, paramterized queries, and multi database support
- :window: Window, aggregate, and ML support for in database analytics
- :rocket: Super compact, storage efficient and compresssed with the use of "lzzzz" compression library, along with rar archiving, and data partitioning

<div>&nbsp;</div>

## **Installation**
Baby Turkey is available on [PyPi](https://pypi.org/project/baby-turkey/) and [Crates.io](https://crates.io/crates/baby_turkey). To install, simply run the following command:

```bash
pip install baby-turkey
```

```bash
cargo install baby_turkey
```

<div>&nbsp;</div>

## **Usage**
Baby Turkey is designed to be easy to use, and is designed to be used in a variety of use cases. The following is a simple example of how to use Baby Turkey in Python:

Python:
```python
from baby_turkey import BabyTurkey
turkey = BabyTurkey()
turkey.create_table("users", ["id", "name", "age"])
turkey.insert("users", {"id": 1, "name": "John", "age": 20})

# Get all users
users = turkey.select("users")
print(users)

# Get all users with age > 18
users = turkey.select("users", "age > 18")
print(users)

# graph query
turkey.create_edge("users", "{id: 1}", "{id: 2}", "friends")
turkey.create_edge("users", "{id: 1}", "{id: 3}", "friends")
traversal = turkey.traverse("users", "{id: 1}", "friends", 1)
print(traversal)

# export db to .turkey file - compressed and archived for use in other projects
turkey.export("users.turkey")

```

Gobble Query Language:
```sql
-- gql create table example
    CREATE TABLE users (id, name, age)
    WITH TABLE NODE AS users.id;

-- gql insert example
    users INSERT {id: 1, name: "John", age: 20}

-- gql select example
    users SELECT age > 18

-- gql traverse example
    EXECUTE TRAVERSAL users FROM {id: 1} WITH friends
    DEPTH 1
    OPTIONAL PARAMS:
        - time_limit: 1000
        - max_depth: 10
        - max_nodes: 1000
        - max_edges: 1000

-- gql stored procedure example
    CREATE PROCEDURE get_users
    WITH PARAMS:
        - age: INT
    RETURNS:
        - users: TABLE
    AS:
        users SELECT age > age

```

Rust:
```rust
use baby_turkey::BabyTurkey;
let mut turkey = BabyTurkey::new();
turkey.create_table("users", vec!["id", "name", "age"]);
turkey.insert("users", vec![1, "John", 20]);

// Get all users
let users = turkey.select("users");
println!("{:?}", users);

// Get all users with age > 18
let users = turkey.select("users", "age > 18");
println!("{:?}", users);

// graph query
turkey.create_edge("users", "{id: 1}", "{id: 2}", "friends");
turkey.create_edge("users", "{id: 1}", "{id: 3}", "friends");
let traversal = turkey.traverse("users", "{id: 1}", "friends", 1);
println!("{:?}", traversal);

// export db to .turkey file - compressed and archived for use in other projects
turkey.export("users.turkey");
```

<div>&nbsp;</div>

## **Contributing**
Baby Turkey is an open source project, and contributions are welcome. If you would like to contribute, please read the [contributing guidelines](contributing.md).

<div>&nbsp;</div>

## **License**
Baby Turkey is licensed under the [MIT License](LICENSE).

<div>&nbsp;</div>

## **Acknowledgements**
- [Dagoba](https://github.com/skvrahul/dagoba) - Inspiration for this project

<div>&nbsp;</div>

## **Contact**
- [Email](mailto:khgardner@pm.me)
- [Twitter](https://twitter.com/tckeezy)
- [LinkedIn](https://www.linkedin.com/in/khlgardner/)

<div>&nbsp;</div>

## **Donate**
If you would like to support this project, please consider donating. All donations are greatly appreciated.