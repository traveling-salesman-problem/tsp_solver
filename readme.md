# Traveling Salesman Problem Solver

## Informations
The following program will help you find a proper solution to a traveling salesman problem.
The program works using a CLI (Command Line Interface).
It uses a genetic algorithm in the backend to find a solution. The dataset can have distances that are not the same in both directions.

## How to compile the code
If you want to compile the code by yourself, just clone the folder, open a shell next to the *Cargo.toml* file, and run `cargo build --release`. The compiled program can be found in the folder `target/release/tsp_solver.exe`.

> :warning: You need to [install Rust](https://www.rust-lang.org) and C++ compiling tools to compile the code, which can be bothersome if you don't use them otherwise. A possibility is to use [Rust's official docker container](https://hub.docker.com/_/rust), that can be deleted once the job is done.

> :information_source: As an alternative, we also provide a precompiled version of the program in the release tab on Github (probably on the right of the page). You just have to download it and decompress it to test our solution.

## Execute the program
The program is a CLI, meaning that you shouldn't just right click on the executable to run it. Open a shell, and run `./tsp_solver.exe -h` to see the list of arguments you can use. Run `./tsp_solver.exe` to run the program on a demo dataset. It will show you the shortest path between french cities (the distances used in the demo dataset aren't the real ones). The `logs.txt` file will contain all the individuals of each generation so you can see how the algorithm performs.