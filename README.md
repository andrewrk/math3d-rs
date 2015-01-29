# math3d-rs

A simple rust library for computer-graphics matrix calculations, because I'm not
smart and/or educated enough to understand the fancy generalized linear algebra
libraries.

This library only has these types:

 * Matrix4
 * Vector4

It only supports operations on these types that I personally understand. If you
make a pull request that adds more linear algebra, you have to teach me what it
does before I'll merge it.

When in doubt, I copied the API and code from
[glm](https://github.com/g-truc/glm/). This library follows the
[right hand rule](https://en.wikipedia.org/wiki/Right-hand_rule), and all angles
are in radians because degrees are for presentation, not math.
