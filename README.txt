Jason Aguirre and Eric Shaw

Help received from Isaac Chen and Connor Gray

Correctly Implemented:
Universal Machine

Significant departures from the design:

Instead of:
instruct.rs - contains code for each instruction that doesn't involve memory.
mem.rs - contains code for managing the memory space.

We set up modules for:
init.rs - initialize the machine
instruct.rs (modified from rumdump lab by Noah Daniels)
rumload.rs - contains code for retrieving instructions for UM file (taken from rumdump lab by Noah Daniels)

System Architecture:
rumload.rs -  the rumload function abstracts the code for retrieving instructions for UM file, init.rs calls this function to fill the program counter memory segment
init.rs - abstracts the code for the initialization of the universal machine, the initialized machine is then modified by the instruct.rs module
instruct.rs - abstracts code for decoding instructions, modifies the state of the machine.

Time to execute 50 million instructions:

Using the following benchmark of midmark.um, we get an execution time of 19.88s.
Since midmark contains ~80 million instructions, we could multiply the execution time by 50/80.
Giving us an execution time of 19.88 * (50/80) = ~12.425s

Benchmark 1: ./target/release/rum ../rum-binaries/midmark.um
  Time (mean ± σ):     19.880 s ±  1.082 s    [User: 17.382 s, System: 1.580 s]
  Range (min … max):   17.636 s … 21.522 s    10 runs

(This benchmark was done on my personal machine)

We spent about 3 hours analyzing the document
We spent 4 hours preparing the doc for this assignment
We spent 6 hours working on the RUM assignment