# rust
1. cargo new <project name>
   Example:
      cargo new fibonocci
   output:
       cd fibonocci; tree
       |-- Cargo.toml
       `-- src
           `-- main.rs
      
2. cargo new empdb
        -> Creates the employee database with name, age, date of joining.

3. cargo new async_program
        -> Extends the empdb with async.
        -> Struct obj are pushed to the vector in one coroutine, and another
           corouting fetches the struct and populates hashmap

4. cell_refcell
        -> Explanations on Cell and RefCell. 
