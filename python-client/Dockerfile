# Start with the official Python image
FROM python:3.11-slim

# Set working directory
WORKDIR /app

# Install dependencies for compiling proto files
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
COPY python-client/requirements.txt /app/requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Copy the proto file into the container
COPY proto/service.proto /app/proto/service.proto

# Generate Python gRPC code using grpcio-tools
RUN python -m grpc_tools.protoc --python_out=/app/python-client/ --grpc_python_out=/app/python-client/ /app/proto/service.proto

# Copy the rest of the Python client code into the container
COPY python-client /app/python-client

# Set the entry point for the client
CMD ["python", "python-client/client.py"]
