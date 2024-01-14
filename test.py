from hex_renderer_python import GridOptions, GridPatternOptions, Point, Color, Lines, Intersections, Marker, PatternVariant, HexGrid
gradient = GridOptions(
    line_thickness=0.12,
    center_dot=Point.None_(),
    pattern_options=GridPatternOptions.Uniform(
        lines=Lines.Gradient(
            colors=[
                Color(214, 9, 177, 255),
                Color(108, 25, 140, 255),
                Color(50, 102, 207, 255),
                Color(102, 110, 125, 255),
            ],
            bent=True,
            segments_per_color=15,
        ),
        intersections=Intersections.UniformPoints(
            point=Point.Single(
                marker=Marker(
                    color=Color(255, 255, 255, 255),
                    radius=0.07,
                ),
            ),
        ),
    ),
)


patterns = [
    PatternVariant("NORTH_EAST", "qaq"),
    PatternVariant("EAST", "aa"),
    PatternVariant("NORTH_EAST", "qaq"),
    PatternVariant("EAST", "wa"),
    PatternVariant("EAST", "wqaawdd"),
    PatternVariant("NORTH_EAST", "qaq"),
    PatternVariant("EAST", "aa"),
    PatternVariant("NORTH_EAST", "qaq"),
    PatternVariant("EAST", "wa"),
    PatternVariant("EAST", "weddwaa"),
    PatternVariant("NORTH_EAST", "waaw"),
    PatternVariant("NORTH_EAST", "qqd")
]

hex_grid = HexGrid(patterns, 50)

hex_grid.draw_to_file("test.png", 50, gradient)
