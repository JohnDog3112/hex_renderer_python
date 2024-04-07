from hex_renderer_py import GridOptions, GridPatternOptions, Point, Lines, Intersections, HexGrid, SquareGrid

monocolor = GridOptions(
    line_thickness=0.12,
    pattern_options=GridPatternOptions.Uniform(
        intersections=Intersections.Nothing(),
        lines=Lines.Monocolor(
            color=palettes["default"][0],
            bent= True
        )
    ),
    center_dot=Point.None_()
)

grid = HexGrid(patterns, 50)

print("Segment")
display(Image(data=bytes(grid.draw_png(50, segment))))

print("Gradient")
display(Image(data=bytes(grid.draw_png(50, gradient))))

print("Monocolor")
display(Image(data=bytes(grid.draw_png(50, monocolor))))