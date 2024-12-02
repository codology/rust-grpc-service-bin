# gRPC Service with Python Client, RabbitMQ, Prometheus, and Grafana

A simple gRPC service in Rust, which processes a 64-byte binary payload received from a Python client. The service writes the payload data to a file on disk and returns a status message. The Python client listens for messages from a RabbitMQ queue, sends the data to the gRPC service, and displays the response.

Also includes Prometheus metrics to monitor the service, which can be visualized using Grafana.

## Layout
```bash
├── docker-compose.yml
├── grpc-service/
│   ├── Dockerfile
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   ├── proto/
│   │   └── service.proto
│   └── target/  # build
├── python-client/
│   ├── Dockerfile
│   ├── requirements.txt
│   └── client.py
├── prometheus.yml  # Configuration for Prometheus
└── .gitignore  # Optional
```

## How It Works

1. **gRPC Service (Rust)**:
   - The Rust service listens for gRPC requests on port `50051`.
   - It receives a `FileRequest` containing a 64-byte binary payload, processes it, writes the data to a file (`output_file.bin`), and responds with a `FileResponse` status message.
   - The service exposes Prometheus metrics on port `9090`.

2. **Python Client**:
   - Auto-generate the code from the protobuff `grpc-service/proto/service.proto` using the command `python -m grpc_tools.protoc -I . --python_out=. --grpc_python_out=.  grpc-service/proto/service.proto` (Note that this is automatically done by docker build)
   - The Python client listens for messages on a RabbitMQ queue (`file_queue`).
   - When a message is received, it sends a request containing the message data (64 bytes) to the gRPC service.
   - After receiving the response from the gRPC service, the client increments a Prometheus counter.

3. **RabbitMQ**:
   - RabbitMQ serves as the message queue to trigger the Python client. The client listens to the queue and processes messages as they arrive.

4. **Prometheus and Grafana**:
   - Prometheus scrapes the gRPC service metrics at `localhost:9090/metrics`.
   - Grafana can be used to visualize the metrics collected by Prometheus.

## Build and start the services

Docker and Docker Compose must be installed. Run the following command to build and start all services (gRPC service, Python client, RabbitMQ, Prometheus, and Grafana):

```bash
    docker-compose up --build
```

This will build the Docker images and start the containers for:

- gRPC server (Rust)
- Python client
- RabbitMQ
- Prometheus
- Grafana

### Access Prometheus and Grafana

* Prometheus: Once the services are up and running, Prometheus will be available at http://localhost:9091.
    
* Grafana: Grafana can be accessed at http://localhost:3000. Log in with the default credentials (admin / admin), and configure Prometheus as a data source (URL: http://prometheus:9090).

### Send messages to RabbitMQ

To trigger the Python client, you can send a message to the file_queue in RabbitMQ. See example in `python-client/send_message.py`

This will trigger the Python client to send the message to the gRPC service, which processes the data.

## Stopping
To stop the services, run:
```bash
    docker-compose down
```