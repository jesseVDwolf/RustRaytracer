# Ray tracer

## Config
The objects can be configured a local JSON file. The path to the JSON file should be passed as an argument to the program.

## Math
We can use the quadratic formula: $x=\frac{-b`\pm\sqrt{b^2-4ac}}{2a}$ to find intersections between rays shot from the camera in objects in the scene. Each object can be described using a mathematical formula.

### Spheres
The base equation for a sphere is: $x^2+y^2+z^2=r^2$ where $r$ is the radius of the sphere.
Above equation assumes the sphere is put at the origin (i.e. $(0, 0, 0)$). If we allow for movement we can adjust the equation to: $(x - h)^2 + (y - k)^2 + (z - l)^2=r^2$ where $(h, k, l)$ represents point $C$, which is the the location of the center of the sphere.

## Sources
* Homogeneous coordinates: https://www.youtube.com/watch?v=o-xwmTODTUI
