# CPTrainBot Backend

This is the Rust backend for CPTrainBot, a Discord bot that retrieves train schedules and information from Infrastruturas de Portugal and displays them on Discord.

## Getting Started

To get started with the backend of CPTrainBot, you'll need to have Rust installed on your system. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository and navigate to the backend directory. From there, you can run `cargo build` to build the project and `cargo run` to start the backend.

## Usage

The backend of CPTrainBot is responsible for retrieving train schedules and information from Infrastruturas de Portugal. It communicates with the Discord bot to provide this information to users.

The backend uses a RESTful API to expose several endpoints for retrieving train schedules and information. These endpoints can be accessed by sending HTTP requests to the appropriate URL.

For example, to retrieve information about a specific train, you can send a GET request to the `/train/:trainnumber` endpoint, where `:trainnumber` is the number of the train you want to retrieve information about.

### (Current!) Endpoints

The backend exposes several endpoints for retrieving train schedules and information:

- `/train/:trainnumber`: Retrieves information about a specific train. This endpoint returns a JSON object containing information about the train's arrival time, departure time, destination, duration, operator, origin, service type, status, and stops.

Documentation for these endpoints is coming soon.

## Contributing

Contributions are welcome! Please feel free to open a pull request or an issue if you have any suggestions or improvements.

## License

This project is licensed under the GNU GPLv3 license. Please refer to the LICENSE file for more information.
