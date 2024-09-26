# path_finder

find executable path

```rust
use path_finder::find_in_env;

fn main() {
    //find in PATH and env vars
    let path = find("cargo");
    println!("{:?}", path);

    //find in PATH, env vars, and cargo home
    let path = find_with_cargo_home("cargo");
    println!("{:?}", path);

    //find in PATH
    let path = find_in_path("cargo");
    println!("{:?}", path);

    //find in env vars
    let path = find_in_env("cargo");
    println!("{:?}", path);
}
```
