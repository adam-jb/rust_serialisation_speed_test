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
Mini network Djikstra benchmark: 2.508606719s
Reached 944 nodes
Loop took 175.922609ms
Vector Serialisation took 590.537725ms
Vector Deserialisation took 355.399577ms
Graph population took 3.033528389s
Graph Serialisation took 944.261254ms
Graph Deserialisation took 1.702373441s
```
