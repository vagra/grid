# grid

uniform-grid and loose/tight double-grid impl for rust and bevy. 

## examples

arrow keys: move current agent
z,x: swhtch current agent

### bevy_ugrid
```
cargo run -r --example bevy_ugrid
```
blue: agent
green: uniform cell
red: current agent bumped other
![bevy_ugrid]()

### bevy_dgrid
```
cargo run -r --example bevy_dgrid
```
blue: agent
green: loose cell
yellow: loose cell aabb rect
gray: tight cell
red: current agent bumped other
![bevy_dgrid]()




## thanks for

- [Dragon Energy@stackoverflow](https://stackoverflow.com/questions/41946007)
- [dragon-space](https://github.com/terrybrash/dragon-space)
- [quadtree](https://github.com/rangercyh/quadtree)
- [aoi](https://github.com/Lyra-Game/aoi)