# A Log Parsing Case Study for Skip Regex

This project generates a bunch of application logs from
Apache Kafka by pumping messages through a cluster with the
log level set to trace on all subsystems, then there is a
rust application which does a bunch of log parsing and
statistics reporting on the results.

Requires:
  1. Apache Kafka to be installed on the system
  2. the `kafka-python` pip package to be installed
