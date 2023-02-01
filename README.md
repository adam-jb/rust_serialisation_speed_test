# rust_serialisation_speed_test

Script to compare rust processes between CE and local machines. Results from 1/2/23:

```
Local:
Mini network Djikstra benchmark: 2.106074333s
Reached 944 nodes
Loop took 385.115041ms
Vector Serialisation took 447.552833ms
Vector Deserialisation took 306.16875ms
Graph population took 3.21979975s
Graph Serialisation took 750.802583ms
Graph Deserialisation took 1.211326334s


CE:
Loop took 235.249075ms
Vector Serialisation took 572.806317ms
Vector Deserialisation took 355.906657ms
Graph population took 2.178187848s
Graph Serialisation took 797.817181ms
Graph Deserialisation took 679.524387ms
```
