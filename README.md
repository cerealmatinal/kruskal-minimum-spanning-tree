# Kruskal's Algorithm:
This is a Rust program that implements Kruskal's Algorithm to find the Minimum Spanning Tree of a given undirected graph.

### Requirements:
 - Rust Compiler (Installation instructions can be found at rust-lang.org)

### Usage:
 - Clone or download the repository to your local machine.
 - Open a terminal in the project directory.
 - Compile the program in release mode with the command cargo build --release.
 - Run the program with the command ./target/release/kruskal <input_file>, where <input_file> is the path to a text file containing the definition of the input graph in the following format:
~~~<number_of_nodes> <number_of_edges>
<node_1> <node_2> <weight>
<node_1> <node_2> <weight>
...
~~~

 - For example:
~~~
4 5
0 1 2
0 2 3
0 3 4
1 3 1
2 3 5
~~~

 - The program will output the edges of the Minimum Spanning Tree in the format:
~~~
<node_1> -- <node_2> : <weight>
<node_1> -- <node_2> : <weight>
...
~~~

For example:
~~~
1 -- 3 : 1
0 -- 1 : 2
0 -- 2 : 3
~~~
