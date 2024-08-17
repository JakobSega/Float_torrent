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

