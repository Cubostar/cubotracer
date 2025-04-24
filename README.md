My personal raytracer. 50% personal project, 50% my baby. Currently finishing up *Raytracing in One Weekend*.
The specific method used to render scenes is backward ray tracing.

TODOs:
- [ ] Add library crate
- [ ] Refactor code for library crate and better organization
- [ ] Dielectrics
- [ ] Optimize computation of object surface normals
- [ ] Smooth shading
- [ ] Improve OBJ support (use tobj?)

Completed Tasks:
- [X] Antialiasing
- [X] Diffuse and Metal Materials
- [X] Positionable Camera
- [X] Defocus Blur
- [X] OBJ support

For EECE5640:

To run examples, use the following command(s):

cargo run --example spheres
cargo run --example stanford-bunny

Note that for the bunny, the command must be run in the examples folder.

To run examples with perf, run the following command(s):

perf stat -d cargo run --profile cuboperf --example spheres
perf stat -d cargo run --profile cuboperf --example stanford-bunny

Again, note that for the bunny, the command must be run in the examples folder.
