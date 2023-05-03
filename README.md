# grid

uniform-grid and loose/tight double-grid impl for rust and bevy. 

## examples

arrow keys: move current agent / camera

z,x: swhtch current agent

### bevy_ugrid
```
cargo run -r --example bevy_ugrid
```
blue: agent

green: uniform cell

red: current agent bumped other

![bevy_ugrid](https://github.com/vagra/grid/blob/9d0cadfe9925d35710dd9e995116895c31f68341/assets/bevy_ugrid.png)

### many_ugrid
```
cargo run -r --example many_ugrid
```

![many_ugrid](https://github.com/vagra/grid/blob/2c0e801ee43a8b66b13b95431770fb91d355b9ef/assets/many_ugrid.png)

### bevy_dgrid
```
cargo run -r --example bevy_dgrid
```
blue: agent

green: loose cell

yellow: loose cell aabb rect

gray: tight cell

red: current agent bumped other

![bevy_dgrid](https://github.com/vagra/grid/blob/9d0cadfe9925d35710dd9e995116895c31f68341/assets/bevy_dgrid.png)


### many_dgrid
```
cargo run -r --example many_dgrid
```
![many_dgrid](https://github.com/vagra/grid/blob/1029a790f531d543e4c34595b2ead3bd9ab4a524/assets/many_dgrid.png)

## thanks for

- [Dragon Energy@stackoverflow](https://stackoverflow.com/questions/41946007)
- [dragon-space](https://github.com/terrybrash/dragon-space)
- [quadtree](https://github.com/rangercyh/quadtree)
- [aoi](https://github.com/Lyra-Game/aoi)