# CPTrainBot Backend

This is the Rust backend for CPTrainBot, a Discord bot that retrieves train schedules and information from Infrastruturas de Portugal and displays them on Discord.

## Prerequisites

To run the backend of CPTrainBot, you'll need to have Rust installed on your system. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

## Installation

To install the backend of CPTrainBot, follow these steps:

1. Clone this repository.
2. Run `cargo build` to build the project.

In the near future, a Docker image that combines both the frontend and backend components of CPTrainBot will be made available for easier deployment.

## Running the Backend

To start the backend of CPTrainBot, run `cargo run` in the project root. The backend server will start and expose its API on port 8000.

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
