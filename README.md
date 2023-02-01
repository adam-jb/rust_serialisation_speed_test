# rust_serialisation_speed_test

Script to compare rust processes between CE and local machines. Results from 1/2/23:

```
Local:
Loop took 374.942667ms
Vector Serialisation took 437.238125ms
Vector Deserialisation took 299.177417ms
Graph population took 2.550919625s
Graph Serialisation took 588.363ms
Graph Deserialisation took 470.972417ms


CE:
Loop took 235.249075ms
Vector Serialisation took 572.806317ms
Vector Deserialisation took 355.906657ms
Graph population took 2.178187848s
Graph Serialisation took 797.817181ms
Graph Deserialisation took 679.524387ms
```
