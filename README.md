# Baby Turkey

## *The all-in-one solution to all things data*

<div>&nbsp;</div>

## **Introduction**
 Baby Turkey is an optimized, robust, easy to use, fully featured database for both transactional and analytical use cases. The database is multi-modal, allowing data to be stored in tabular, document, and graph format. The database features a rich unique query language similar to traditional SQL, along with in database machine learning, in database visualization, rich graph analytics, and a balance of read/write performance and storage efficiency.
 Baby Turkey also includes a one-click deployment solution for both local and cloud environments. The deployment solution is fully customizable, allowing users to deploy Baby Turkey on any cloud provider, or on any local machine. The deployment solution also includes a fully featured web interface for database management, query execution, and data visualization. Finally, the database is fully open source, allowing users to modify the database to their needs, along with having an open-source low code visual api development kit, allowing users to create APIs for their data and power their applications with ease.

<div>&nbsp;</div>

## **Features**
- :graph: In database visualization tools, can create visualizations with the query language
- :chart_with_upwards_trend: In database machine learning, can train models with the query language
- :file_folder: Multi-modal, can store data in tabular, document, and graph format, able to query the same data entity in all three formats
- :mag: Rich query language, similar to traditional SQL, but with many more features and capabilities
- :rocket: Optimized, able to handle large amounts of data, and able to handle complex queries, built on top of Rust and Python
- :lock: Robust, able to handle many concurrent users, and able to handle many concurrent queries
- :computer: Easy to use, able to be used with a variety of programming languages, and able to be used with a variety of tools
- :cloud: One-click deployment solution, able to deploy Baby Turkey on any cloud provider, or on any local machine
- :open_file_folder: Fully open source, able to modify the database to your needs, and able to modify the deployment solution to your needs
- :hammer: Low code visual api development kit, able to create APIs for your data, and able to power your applications with ease

<div>&nbsp;</div>

## **Installation**
To install Baby Turkey, you can download the latest release from the [releases page](releases). You can also install Baby Turkey from source by cloning the repository and running `make install`. To run Baby Turkey, you can run `babyturkey` from the command line.

Baby Turkey language packages are available on [PyPi](https://pypi.org/project/baby-turkey/) and [Crates.io](https://crates.io/crates/baby_turkey). To install, simply run the following command:

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