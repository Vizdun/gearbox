# Gear Types

Gear
Counter
MalteseCross/GenevaDrive
Ender

# Syntax

l - label

example

```
g12g6c6"ABC"e6
```

gear with 12 teeth followed by gear with 6 followed by counter ABC

pararel example

```
g12[g2l"Gear 1:"c2"ABC",g4l"Gear 2:"c4"ABC"]g6l"Gear3:"c6"ABC"e6
```

# Major TODOs

MalteseCross

# Equation

for index > 1
(disk_size-1)*(ender_size/index)
for index 1
(disk_size-1)*(ender_size/2)
for index 0
(disk_size-1)*(ender_size*2)