# rust_serialisation_speed_test

Script to compare rust processes between CE and local machines. Results from 1/2/23:

```
Local:
Mini network Djikstra benchmark: 191.40475ms
Reached 944 nodes x1000 times
Loop took 42ns
Vector Serialisation took 4.464417ms
Vector Deserialisation took 20.223583ms
Graph population took 2.035119541s
Graph Serialisation took 199.417417ms
Graph Deserialisation took 1.533202667s


CE:
Mini network Djikstra benchmark: 197.890435ms
Reached 944 nodes x1000 times
Loop took 63ns
Vector Serialisation took 12.032993ms
Vector Deserialisation took 36.731377ms
Graph population took 2.995024599s
Graph Serialisation took 364.759121ms
Graph Deserialisation took 2.839592218s
```
