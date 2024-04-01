from hex_renderer_python import GridOptions, GridPatternOptions, Point, Color, Lines, Intersections, Marker, AngleSig, PatternVariant, HexGrid, EndPoint



#grid_options = GridOptions(
#    line_thickness=0.11,
#    pattern_options=GridPatternOptions.Changing(
#
#    ),
#    center_dot=Point.None_()
#)

pattern_list = [
    PatternVariant("North_East", "qqq"),
    PatternVariant("South_West", "qqq")
]

grid = HexGrid(pattern_list, 50)

#grid.draw_to_file("example.png", 50, grid_options)