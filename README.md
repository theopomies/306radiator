# 306 RADIATOR

A radiator is placed in a square room. We want to know how the temperature is distributed in the room based on the radiator’s location. The temperature in the room verifies the very famous heat equation.

## How to build
```sh
$ make re
```

## Examples
```
./306radiator -h
USAGE
    ./306radiator n ir jr [i j]
DESCRIPTION
    n           size of the room
    (ir, jr)    coordinates of the radiator
    (i, j)      coordinates of a point in the room
```

````
./306radiator 4 1 1 
1   0   0   0   0   0   0   0   0   0   0   0   0   0   0   0
0   1   0   0   0   0   0   0   0   0   0   0   0   0   0   0
0   0   1   0   0   0   0   0   0   0   0   0   0   0   0   0
0   0   0   1   0   0   0   0   0   0   0   0   0   0   0   0
0   0   0   0   1   0   0   0   0   0   0   0   0   0   0   0
0   4   0   0   4   -16 4   0   0   4   0   0   0   0   0   0  
0   0   4   0   0   4   -16 4   0   0   4   0   0   0   0   0
0   0   0   0   0   0   0   1   0   0   0   0   0   0   0   0
0   0   0   0   0   0   0   0   1   0   0   0   0   0   0   0
0   0   0   0   0   4   0   0   4   -16 4   0   0   4   0   0
0   0   0   0   0   0   4   0   0   4   -16 4   0   0   4   0
0   0   0   0   0   0   0   0   0   0   0   1   0   0   0   0
0   0   0   0   0   0   0   0   0   0   0   0   1   0   0   0
0   0   0   0   0   0   0   0   0   0   0   0   0   1   0   0
0   0   0   0   0   0   0   0   0   0   0   0   0   0   1   0
0   0   0   0   0   0   0   0   0   0   0   0   0   0   0   1

0.0
0.0
0.0
0.0
0.0
21.9
6.3
0.0
0.0
6.3
3.1
0.0
0.0
0.0
0.0
0.0
```

```
./306radiator 4 1 1 2 2
3.1
```

```
./306radiator 5 1 2 3 2
3.3
```

```
./306radiator 8 4 6 3 6
9.4
```

```
./306radiator 12 3 9 1 6
2.5
```