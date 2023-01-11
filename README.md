# mathematica

A bunch of useful mathematical functions and operations, in rust!!

for now there's only vectors & matrices

also i am very less familiar with rust workspaces... i wanted to have the following code to be in a workspace and in bunch on sub crates like this:

```txt
mathematica
          |
          |-Cargo.toml
          |-Cargo.lock
          |-src
          |   |-main.rs // which can import the matrix and vector crate (idk how to please help)
          |
          |-lib
              |-vectors
              |       |
              |       // files related to it (idk how)
              |
              |-matrices
                       |
                       // files related to it (idk how)
```

in the end idk how the manifest works and how to import these, so if u can help please do :(...
