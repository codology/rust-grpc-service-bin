import pika

connection = pika.BlockingConnection(pika.ConnectionParameters('localhost'))
channel = connection.channel()

# Declare the queue
channel.queue_declare(queue='file_queue')

# Send a 64-byte message
message = b'1234567890' * 6  # 64 bytes of data
channel.basic_publish(exchange='', routing_key='file_queue', body=message)

print("Sent message to RabbitMQ")
connection.close()
