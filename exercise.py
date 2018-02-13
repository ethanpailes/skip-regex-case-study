#!/usr/bin/env python3
#
# File: exercise.py
# Author: Ethan Pailes
#
# This guy just exercises Apache Kafka in order to generate
# a bunch of application logs.
#

from pykafka import KafkaClient
import pdb

def main():
    client = KafkaClient(hosts="localhost:9092")
    test_topic = client.topics[b"test"]

    with test_topic.get_sync_producer() as p:
        produce(p, 100)

    consume_all(test_topic)

def produce(p, no_msg):
    """ Produce `no_msg` messages """
    for i in range(no_msg):
        p.produce(bytes("test message " + str(i), encoding="utf8"))

def consume_all(t):
    """ Consume all the messages in the topic at the time that this
        function is called, then return.
    """
    c = t.get_simple_consumer()
    last_off = t.partitions[0].latest_available_offset() - 1
    for m in c:
        if m is not None:
            print("{}: {}".format(m.offset, m.value))
        if m.offset >= last_off:
            break

if __name__ == "__main__":
    main()

