# bevy_svg

For one of my personal projects i needed a way to load and display some simple SVG files/shapes in [`Bevy`],
so i took inspiration from [`bevy_prototype_lyon`] and modified and extended it to...well...load and display
simple SVG files. It currently is rather limited in drawing the lines in the correct color, it just uses the
first occuring color in the file.

Ideally, this will change in the future. Currently i use [`usvg`] to load, parse and simplify an SVG or SVGZ file
and afterwards use [`Lyon`] to tessellate and draw it as a [`Bevy`] mesh. For the color, the first occurance of a
color in the file is used as the material for the resulting mesh, which is why every path on the mesh currently
has the same color.

In the future i want to change this, maybe use shader to draw the correct color of a [vertex](https://github.com/Weasy666/bevy_svg/blob/master/src/plugin.rs#L39) or doing
some kind of UV magic...dunno, but if someone wants to tackle this, go for it, i will happily accept such a PR,
or PRs in general.

[`Bevy`]: https://bevyengine.org
[`bevy_prototype_lyon`]: https://github.com/Nilirad/bevy_prototype_lyon
[`Lyon`]: https://github.com/nical/lyon
[`usvg`]: https://github.com/RazrFalcon/resvg
