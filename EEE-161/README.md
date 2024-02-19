
# CoE 161: Information and Complexity

> Advanced course on information theory and computational complexity, starting from Shannon's information theory and Turing's theory of computation, leading to the theory of Kolmogorov complexity.

**Pre-requisites**: EEE 111 (Computation) and EEE 137 (Probability Theory)

**Course Credit**: 3 units

**Course Assessment**: 2 exams (35% each), HWs and Quizzes (25%), Attendance (5%)

## Course Content

| Long Exam 1 | Long Exam 2 |
| --------------------------------------| -------------------------- |
| 1. Introduction to Information Theory | 5. Reliable Communication<br>- Capacity Analysis<br>- Basic Coding Theory |
| 2. Probability Theory | 6. Computational Models (Focus on Turing Machines) |
| 3. Information Measures | 7. Complexity Classes |
| 4. Compression<br>- Symbol Codes<br>- Block Codes<br>- Stream Codes | 8. Algorithmic Information (Kolmogorov Complexity) |


## Information

In 1948, Claude Shannon published *"A Mathematical Theory of Communication"* which aimed to solve following three questions:

> 1. How to **measure** information?
> - Bits = "Currency" of information
> - Asking the most "Informative" Question through ML/Data Science

> 2. How to **compress** information?
> - lower redundancy
> - lossless (png, zip) vs lossy (jpg, mp3)

> 3. How to **reliably** send information?
> - channels are unreliable
> - higher redundancy

## Complexity

*"Church-Turing Thesis"* written by Alonzo Church and Alan Turing explained that any reliazable computation device can be simulated by a Turing machine. It aimed to answer the following questions:

> 1. How to *Mathematically* **model** a Computer?
> - With internal states (i.e. Finite State Machines or FSM)
> - Using stack memory (i.e. Pushdown Automata or PDA)
> - Using tape memory (i.e. Turing Machine)

> 2. Is a problem **solvable** by a Computer?
> - Are there computational resources needed? (time, memory)
> - Complexity Classes (P, NP, NP-Complete)

## Course Goals:

- Introduce Mathematical Tools and Frameworks to understand Engineering Systems from an Information-Theoretic viewpoint

- Introduce Fundamental Tools for determining the required Computational Resources needed to algorithmically solve a problem

## References

### Information
- Elements of Information Theory by Cover and Thomas '03
- Information Theory, Learning, and Inference by David Mckay '03

### Computation
- Introduction to Theory of Computation by Michael Sipser '13
- Computation Complexity: A Modern Approach by Boaz Barak and Sajeev Arora '07