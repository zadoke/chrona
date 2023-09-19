# chrona

chrona is a Rust-based service/backend designed for interfacing with the Infrastruturas de Portugal API. Its primary function is to retrieve, process, and reformat train schedule and information data into a format that is easier for developers to work with the API.

## Get Started

There are 3 ways you can run chrona:

## 1. Use Docker

The docker image for chrona is hosted on [Docker Hub](https://hub.docker.com/repository/docker/zadoke/chrona/)

To run it, you can run the following command:

`docker run -p 8000:8000 zadoke/chrona`

You can also use the SERVER_PORT environment variable to specify the port number on the container. For example, the following command will run chrona on port 9000 on the container and map it to port 8000 on the host:

`docker run -p 8000:9000 -e SERVER_PORT=9000 zadoke/chrona`

## 2. Run locally

To run locally, you'll need to have Rust installed on your system. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Clone the repository and run:

`cargo run`

The server will then be listening on http://0.0.0.0:8000.

You can change the default port by modfiying the SERVER_PORT environment variable.

## 3. Deploy to Kubernetes

Refer to [deployments!](https://github.com/zadoke/chrona/tree/main/deployments)

## Usage

chrona's core responsibility is to facilitate the retrieval and transformation of train schedule and information data sourced from Infrastruturas de Portugal. Its primary audience includes applications requiring streamlined access to this data.

chrona uses a RESTful API to expose several endpoints for retrieving train schedules and information. These endpoints can be accessed by sending HTTP requests to the appropriate URL.

For example, to retrieve information about a specific train, you can send a GET request to the `/train/:trainnumber` endpoint, where `:trainnumber` is the number of the train you want to retrieve information about.

### Endpoints

You can check the current endpoints in the [wiki!](https://github.com/zadoke/chrona/wiki/Endpoints)

## Contributing

Contributions are welcome! Please feel free to open a pull request or an issue if you have any suggestions or improvements.

## License

This project is licensed under the GNU GPLv3 license. Please refer to the LICENSE file for more information.
