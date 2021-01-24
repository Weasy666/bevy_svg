# bevy_svg

For one of my personal projects i needed a way to load and display some simple SVG files/shapes in [`Bevy`],
so i took [`bevy_prototype_lyon`] as inspiration and modified and extended it to...well...load and display
simple SVG files. It currently is rather limited, it loads all path in a SVG, but applies the color of the last
`fill` to every path.

Ideally, this will change in the future. The best option would be to exchange [`Lyon`] with [`resvg`], so that
all the SVG edge cases and complicated stuff does not need to be reinvented.

[`Bevy`]: https://bevyengine.org
[`bevy_prototype_lyon`]: https://github.com/Nilirad/bevy_prototype_lyon
[`Lyon`]: https://github.com/nical/lyon
[`resvg`]: https://github.com/RazrFalcon/resvg
