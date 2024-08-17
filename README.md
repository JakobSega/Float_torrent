# Float_torrent

In this project we desired to write servers, which can handle computations regarding sequences.

# User instructions
The code comes with 3 similar servers. All of them offer the same sequences (but with different names), they only differ in their names. The original server's name is Binarni Banditi, the second one is named Binarni Banditi & AmondUs and the third one is names Binarni Banditi & Elves. To use these servers, first run the register, then run all of the main.rs files in the three generator folders. Then you can send requests via the try.py file. There are some test in this script, so when you run it some tests will execute and will let you know if everything is working as it should. Then you can alter the requests in the python script to send other (more complex) sequences to these servers.

# How the family of servers works
You can just run all servers out of the box. They differ in their ip, port, name and the names of the sequences. For example, the Arithmetic sequence is available on the original server aa 'Arithmetic', on the AmongUs server it is under the name 'Arithemtic_Imposter' and on the Elves server it is under the name 'Arithmetic_Elves'. This is by design, so we can test whether the servers can communicate correctly ie. try to request the 'Arithmetic_Imposter' sequence from the original server and see whether it returns the output, even though this sequence is not available on that server, but is on another one. 

You can also run each server with a custom ip, port and register ip via the comman 'cargo run register_ip generator_ip generator_port'.

# Server's capabilities
The server offers some basic capabilities such as responding to /ping requests, /sequence requests, which gives the user the list of sequences available on the server. The main functionality comes from the POST requests. With a POST you may request some elements of some sequence from the server, following the examples from the python script. 

Here's the updated README for your project, reflecting the changes and improvements made in the Rust server code:

---

# Float Torrent

## Overview

This project implements a set of servers designed to handle computations related to sequences. The servers differ in their configuration and functionality but share the common purpose of sequence handling.

## Server Types

There are three distinct servers, each with a different name and functionality:

1. **Binarni Banditi**: The original server.
2. **Binarni Banditi & AmongUs**: A variant server with its own sequence names.
3. **Binarni Banditi & Elves**: Another variant server with a different set of sequence names.

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

## Customization

You can configure each server to use a specific IP and port by providing these parameters in the run command. Additionally, the register server IP and port can be customized to match your environment.

## Error Handling

If the server encounters issues with port numbers or IP addresses, appropriate error messages will be displayed, and the server will exit if critical issues are encountered.

## Conclusion

This project is designed to test server-to-server communication and sequence handling capabilities. By running the servers with different configurations, you can verify their functionality and ensure proper sequence management.

For further details on sequence requests and server interactions, refer to the `try.py` script included in the project.

---

Feel free to adjust the README further if there are additional details or changes you would like to include!
