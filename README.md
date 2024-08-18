# Float Torrent

## Overview

This project implements a set of servers designed to handle computations related to sequences. The servers differ in their configuration and functionality but share the common purpose of sequence handling.

## Server Types

There are three distinct servers, each with a different name and functionality:

1. **Binarni Banditi**: The original server.
2. **Binarni Banditi & AmongUs**: A variant server with its own sequence names.
3. **Binarni Banditi & Elves**: Another variant server with a different set of sequence names.

## Available Sequences

The server implements a variety of sequences, each with its specific characteristics, parameters, and operations. Below is a detailed description of each sequence:

1. **Arithmetic Sequence**
   - **Name:** `Arithmetic`
   - **Description:** Generates an arithmetic sequence. The first parameter (`a_0`) is the starting element, and the second parameter (`d`) is the common difference. The sequence is defined by the formula: \( a_n = a_0 + n \times d \).
   - **Parameters:** 2
   - **Required Sequences:** 0

2. **Constant Sequence**
   - **Name:** `Constant`
   - **Description:** Generates a constant sequence where every term is the same. The single parameter specifies the constant value.
   - **Parameters:** 1
   - **Required Sequences:** 0 (1 if the server is set as `Imposter`)

3. **Linear Combination Sequence**
   - **Name:** `LinearCombination`
   - **Description:** Combines two sequences linearly. It accepts three scalar parameters (`l_0`, `l_1`, `l_2`) and two sequence parameters. The sequence is defined by: \( c_n = l_0 + l_1 \times a_n + l_2 \times b_n \).
   - **Parameters:** 3
   - **Required Sequences:** 2

4. **Sum Sequence**
   - **Name:** `Sum`
   - **Description:** Computes the element-wise sum of two sequences. The sequence is defined by: \( c_n = a_n + b_n \).
   - **Parameters:** 0
   - **Required Sequences:** 2

5. **Product Sequence**
   - **Name:** `Product`
   - **Description:** Computes the element-wise product of two sequences. The sequence is defined by: \( c_n = a_n \times b_n \).
   - **Parameters:** 0
   - **Required Sequences:** 2

6. **Drop Sequence**
   - **Name:** `Drop`
   - **Description:** Drops the first `i` elements from a sequence. The sequence is defined by: \( c_n = a_{n+i} \).
   - **Parameters:** 1 (Drop count)
   - **Required Sequences:** 1

7. **Geometric Sequence**
   - **Name:** `Geometric`
   - **Description:** Generates a geometric sequence. It accepts two scalar parameters (`a`, `q`). The sequence is defined by: \( c_n = a \times q^n \).
   - **Parameters:** 2
   - **Required Sequences:** 0

8. **Fibonacci Sequence**
   - **Name:** `Fibonacci`
   - **Description:** Generates a Fibonacci sequence. It takes two scalar parameters (`a_0`, `a_1`). The sequence is recursively defined by: \( c_0 = a_0 \), \( c_1 = a_1 \), and \( c_n = c_{n-1} + c_{n-2} \) for \( n > 1 \).
   - **Parameters:** 2
   - **Required Sequences:** 0

9. **Euler-Mascheroni Sequence**
   - **Name:** `EMSequence`
   - **Description:** Approximates the Euler-Mascheroni constant. The sequence is defined by: \( c_n = H(n) - \log(n) \), where \( H(n) \) is the partial sum of the harmonic series.
   - **Parameters:** 0
   - **Required Sequences:** 0

10. **Nth Root Sequence**
    - **Name:** `NthRootSequence`
    - **Description:** Computes the nth root of the elements in a sequence. The sequence is defined by: \( c_n = (a_n)^{1/n} \).
    - **Parameters:** 0
    - **Required Sequences:** 1

11. **Hofstadter Sequence**
    - **Name:** `Hofstadter`
    - **Description:** Generates the Hofstadter's Q-sequence. It is recursively defined as: \( G_0 = 0 \) and \( G_n = n - G(G(n-1)) \) for \( n > 0 \).
    - **Parameters:** 0
    - **Required Sequences:** 0

12. **Recaman Sequence**
    - **Name:** `Recaman`
    - **Description:** Generates the Recaman sequence. It is defined as follows: \( c_0 = 0 \), and \( c_n = a_{n-1} - n \) if the result is positive and has not appeared before, otherwise \( c_n = a_{n-1} + n \).
    - **Parameters:** 0
    - **Required Sequences:** 0

13. **Story Sequence**
    - **Name:** `Story`
    - **Description:** Generates a short story using the GROQ API, based on the specified author and genre. The story is encoded in binary.
    - **Parameters:** 2 (Author, Genre)
    - **Required Sequences:** 1

14. **AI Prediction Sequence**
    - **Name:** `Ai`
    - **Description:** Uses AI to predict the next sequence elements based on the input sequence and the specified prediction range.
    - **Parameters:** 1 (Prediction Range)
    - **Required Sequences:** 1

## Running the Servers

1. **Register Server**: Before starting the individual servers, ensure the register server is running.
2. **Start the Servers**: Run the main executable for each server. Use the following command to start a server:

   ```bash
   cargo run -- [dynamic] IP_REGISTRAR IP_GENERATOR [PORT]
   ```

   - `[dynamic]`: Optional keyword to use dynamic port assignment.
   - `IP_REGISTRAR`: IP address of the registration server.
   - `IP_GENERATOR`: IP address of the current server.
   - `[PORT]`: Optional port number for the server.

   **Examples**:
   - Run with default port: `cargo run`
   - Run with a specific port: `cargo run -- 127.0.0.1 127.0.0.1 8080`
   - Run with dynamic port: `cargo run -- dynamic 127.0.0.1 127.0.0.1`

   If using dynamic port assignment, the server will choose an available port automatically.

3. **Testing with Python**: After starting the servers, you can use the provided `try.py` script to test server functionality. This script includes tests to verify if the servers are operating correctly. You can also modify the script to send custom sequence requests.

## Server Functionality

### Endpoints

- **`GET /ping`**: Responds with information about the server.
- **`GET /sequence`**: Returns a list of sequences available on the server.
- **`POST /sequence`**: Requests specific elements of a sequence. Refer to the Python script for examples of request formatting.

### Server Configuration

- **IP Address and Port**: Each server can be configured with a custom IP address and port.
- **Dynamic Port Assignment**: Optionally, servers can be started with dynamic port assignment if specified with the `dynamic` keyword.

### Sequence Naming

Each server handles sequences under different names:
- **Arithmetic Sequence**:
  - **Original Server**: Named `Arithmetic`
  - **AmongUs Server**: Named `Arithmetic_Imposter`
  - **Elves Server**: Named `Arithmetic_Elves`

This naming convention allows testing of inter-server communication, ensuring that sequences are properly recognized across different servers.

## Error Handling

If the server encounters issues with port numbers or IP addresses, appropriate error messages will be displayed, and the server will exit if critical issues are encountered.

## Conclusion

This project is designed to test server-to-server communication and sequence handling capabilities. By running the servers with different configurations, you can verify their functionality and ensure proper sequence management.

For further details on sequence requests and server interactions, refer to the `try.py` script included in the project.

