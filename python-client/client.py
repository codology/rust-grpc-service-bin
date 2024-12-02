import grpc
import pika
import service_pb2
import service_pb2_grpc
from prometheus_client import start_http_server, Counter
import time

# Prometheus metrics
file_processing_counter = Counter('file_processing_counter', 'Total number of processed files')

def on_message(ch, method, properties, body):
    # Connect to gRPC server
    channel = grpc.insecure_channel('localhost:50051')
    stub = service_pb2_grpc.FileServiceStub(channel)

    # Create request with 64-byte payload
    request = service_pb2.FileRequest(data=body)
    
    # Call the gRPC service
    response = stub.ProcessFile(request)
    print("Received message:", response.status)
    
    # Increment Prometheus counter
    file_processing_counter.inc()

def start_rabbitmq_listener():
    # Connect to RabbitMQ and start consuming messages
    connection = pika.BlockingConnection(pika.ConnectionParameters('localhost'))
    channel = connection.channel()

    # Declare queue
    channel.queue_declare(queue='file_queue')

    # Start consuming messages
    channel.basic_consume(queue='file_queue', on_message_callback=on_message, auto_ack=True)
    print('Waiting for messages. To exit press CTRL+C')
    channel.start_consuming()

if __name__ == "__main__":
    # Start Prometheus metrics server
    start_http_server(8000)

    # Start RabbitMQ listener
    start_rabbitmq_listener()
