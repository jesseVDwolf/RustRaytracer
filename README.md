# Ray tracer

## Requirements
- [x] Ray-Sphere intersection
- [ ] Ray-Plane intersection
- [ ] Lighting in scene
- [ ] Shadows are rendered
- [x] Raylib windowing
- [x] move camera around

## Config
The objects can be configured a local JSON file. The path to the JSON file should be passed as an argument to the program.

## Math
We can use the quadratic formula: $x=\frac{-b`\pm\sqrt{b^2-4ac}}{2a}$ to find intersections between rays shot from the camera in objects in the scene. Each object can be described using a mathematical formula.

### Spheres
The base equation for a sphere is: $x^2+y^2+z^2=r^2$ where $r$ is the radius of the sphere.
Above equation assumes the sphere is put at the origin (i.e. $(0, 0, 0)$). If we allow for movement we can adjust the equation to: $(x - h)^2 + (y - k)^2 + (z - l)^2=r^2$ where $(h, k, l)$ represents point $C$, which is the the location of the center of the sphere. Since we're working with vectors we can write this as $(C - O) \cdot (C - O)=r^2$.

So we end up with two formule:
- $(C - O) \cdot (C - O)=r^2$
- $f(x)=td + Q$

Since $O$ represents a pont on the sphere we can substitute $f(x)=td + Q$ with $O$:
- $(C - (td + Q)) \cdot (C - (td + Q)) = r^2$
- $(-td + (C - Q)) \cdot (-td + (C - Q)) = r^2$
- $t^2d \cdot d - 2td \cdot (C - Q) + (C - Q) \cdot (C - Q) = r^2$
  
So from here we can see:
- $a = d \cdot d$
- $b = -2d \cdot (C - Q)$
- $c = (C - Q) \cdot (C - Q)$

### Planes
You can describe infinity planes using an origin $O$ and orientation $N$. To describe a point on the plane you can use the property of the dot product $\cdot$ that says that two perpendicular vectors should have a dot product of 0.

The formula is as follows:  
$(\vec{x}-\vec{P})\cdot\vec{N}=0$  

Where $\vec{x}$ is the "potential" point on the plane, $\vec{P}$ is the origin of the plane (could be any point on the plane) and $\vec{N}$ is the orientation of the plane.

We can then subsitute the ray formula $f(x)=t\vec{d}+Q$ into the formula and solve for t:
- $((t\vec{d} + Q) - \vec{P})\cdot\vec{N}=0$
- $(t\vec{d} + (Q - \vec{P}))\cdot\vec{N}=0$
- $((t\vec{d})\cdot\vec{N}) + (Q - \vec{P}) \cdot \vec{N}=0$
- $t(\vec{d}\cdot\vec{N}) + (Q - \vec{P}) \cdot \vec{N}=0$
- $t(\vec{d}\cdot\vec{N})=-((Q - \vec{P}) \cdot \vec{N})$
- $t=\frac{-(Q - \vec{P}) \cdot \vec{N}}{(\vec{d}\cdot\vec{N})}$
- $t=\frac{(\vec{P} - Q) \cdot \vec{N}}{(\vec{d}\cdot\vec{N})}$

## Sources
* Homogeneous coordinates: https://www.youtube.com/watch?v=o-xwmTODTUI
* Raytracing guid: https://raytracing.github.io/books/RayTracingInOneWeekend.html
