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
Mini network Djikstra benchmark: 2.545097372s
Reached 944 nodes x1000 times
Loop took 180.231791ms
Vector Serialisation took 585.61857ms
Vector Deserialisation took 360.776207ms
Graph population took 30.865608843s
Graph Serialisation took 9.415347655s
Graph Deserialisation took 17.250857853s
```
