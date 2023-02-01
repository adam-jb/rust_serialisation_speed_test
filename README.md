# rust_serialisation_speed_test

Script to compare rust processes between CE and local machines. Results from 1/2/23:

```
Local:
Mini network Djikstra benchmark: 2.094822458s
Reached 944 nodes x1000 times
Loop took 377.312167ms
Vector Serialisation took 446.332875ms
Vector Deserialisation took 310.63925ms
Graph population took 31.846261542s
Graph Serialisation took 7.848828541s
Graph Deserialisation took 12.947898958s


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
