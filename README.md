# grid

uniform-grid and loose/tight double-grid impl for rust and bevy. 

## examples

```
arrow keys: move current agent / camera
z, x:       swhtch current agent
```

### bevy_ugrid
```
cargo run -r --example bevy_ugrid

blue:   agent
green:  uniform cell
red:    current agent bumped other
```

![bevy_ugrid](https://github.com/vagra/grid/blob/cbdb548c03020fe320a7dffd990db29beb6671c2/assets/bevy_ugrid.png)

### many_ugrid
```
cargo run -r --example many_ugrid
```

![many_ugrid](https://github.com/vagra/grid/blob/cbdb548c03020fe320a7dffd990db29beb6671c2/assets/many_ugrid.png)

### bevy_dgrid
```
cargo run -r --example bevy_dgrid

blue:   agent
green:  loose cell
yellow: loose cell aabb rect
gray:   tight cell
red:    current agent bumped other
```

![bevy_dgrid](https://github.com/vagra/grid/blob/cbdb548c03020fe320a7dffd990db29beb6671c2/assets/bevy_dgrid.png)


### many_dgrid
```
cargo run -r --example many_dgrid
```
![many_dgrid](https://github.com/vagra/grid/blob/cbdb548c03020fe320a7dffd990db29beb6671c2/assets/many_dgrid.png)

## thanks for

- [Dragon Energy@stackoverflow](https://stackoverflow.com/questions/41946007)
- [dragon-space](https://github.com/terrybrash/dragon-space)
- [quadtree](https://github.com/rangercyh/quadtree)
- [aoi](https://github.com/Lyra-Game/aoi)