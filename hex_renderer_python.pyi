class PatternVariant(object):
	"""
	A hexpattern that can be rendered on a grade
	"""
	def __init__(self, direction: str, angle_sigs: str, great_spell: None | bool = None) -> None:
		"""
		Creates a new PatternVariant
		 :param direction: Starting direction (North_East, East, South_East, South_West, West, North_West)
		 :param angle_sigs: String of angle sigs (accepted characters: [q,w,e,d,s,a])
		 :param great_spell: Whether or not it's a great spell (Default = false)
		"""
		...
	@property
	def direction(self) -> str:
		"""
		Gets the starting direction of the pattern 
		"""
		...
	@property
	def angle_sigs(self) -> str:
		"""
		Gets the angle_sigs of the pattern
		"""
		...
	@property
	def great_spell(self) -> bool:
		"""
		Gets whether or not the pattern is a great spell
		"""
		...
	...
class GridOptions(object):
	"""
	Main struct for all pattern rendering options
	"""
	def __init__(self, line_thickness: float, pattern_options: GridPatternOptions.Uniform | GridPatternOptions.Changing, center_dot: Point.None_ | Point.Single | Point.Double) -> None:
		"""
		Main struct for all pattern rendering options
		:param line_thickness: Thickness of line in relation to distance between points
		:param pattern_options: Further options for how to render each pattern
		:param center_dot: Optional point to place in the center of each pattern (helps with determining pattern size at a glance)
		"""
		...
	@property
	def line_thickness(self) -> float:
		"""
		Thickness of line in relation to distance between points
		 eg. if the line_thickness = 0.1, and the distance between points is 10 pixels,
		 then the line_thickness would be 1 pixel
		"""
		...
	@property
	def pattern_options(self) -> GridPatternOptions.Uniform | GridPatternOptions.Changing:
		"""
		Further options for how to render each pattern
		"""
		...
	@property
	def center_dot(self) -> Point.None_ | Point.Single | Point.Double:
		"""
		Optional point to place in the center of each pattern (helps with determining pattern size at a glance)
		"""
		...
	def with_line_thickness(self, line_thickness: float) -> GridOptions:
		...
	def with_pattern_options(self, pattern_options: GridPatternOptions.Uniform | GridPatternOptions.Changing) -> GridOptions:
		...
	def with_center_dot(self, center_dot: Point.None_ | Point.Single | Point.Double) -> GridOptions:
		...
	...
class GridPatternOptions:
	class Changing(object):
		"""
		Changes what pattern renderer to use when finding an introspect or retrospect pattern
		 That way you can change colors/renderers for embedded patterns
		"""
		def __init__(self, variations: list[tuple[Intersections.Nothing | Intersections.UniformPoints | Intersections.EndsAndMiddle, Lines.Monocolor | Lines.Gradient | Lines.SegmentColors]], intros: list[AngleSig], retros: list[AngleSig]) -> None:
			"""
			Changes what pattern renderer to use when finding an introspect or retrospect pattern
			 That way you can change colors/renderers for embedded patterns
			:param variations: Variations to use, starts at the first and goes up when it reaches an intro, goes down when reaching a retro
			:param intros: Vec of the angle_sigs of intro patterns
			:param retros: Vec of angle_sigs of retro patterns
			"""
			...
		@property
		def variations(self) -> list[tuple[Intersections.Nothing | Intersections.UniformPoints | Intersections.EndsAndMiddle, Lines.Monocolor | Lines.Gradient | Lines.SegmentColors]]:
			"""
			Variations to use, starts at the first and goes up when it reaches an intro, goes down when reaching a retro
			"""
			...
		@property
		def intros(self) -> list[AngleSig]:
			"""
			Vec of the angle_sigs of intro patterns
			"""
			...
		@property
		def retros(self) -> list[AngleSig]:
			"""
			Vec of angle_sigs of retro patterns
			"""
			...
		def with_variations(self, variations: list[tuple[Intersections.Nothing | Intersections.UniformPoints | Intersections.EndsAndMiddle, Lines.Monocolor | Lines.Gradient | Lines.SegmentColors]]) -> GridPatternOptions.Changing:
			...
		def with_intros(self, intros: list[AngleSig]) -> GridPatternOptions.Changing:
			...
		def with_retros(self, retros: list[AngleSig]) -> GridPatternOptions.Changing:
			...
		...
	class Uniform(object):
		"""
		Uniform means that all patterns will be rendered in the same way
		 (This excludes the difference with PatternVariant)
		"""
		def __init__(self, intersections: Intersections.Nothing | Intersections.UniformPoints | Intersections.EndsAndMiddle, lines: Lines.Monocolor | Lines.Gradient | Lines.SegmentColors) -> None:
			"""
			Uniform means that all patterns will be rendered in the same way
			 (This excludes the difference with PatternVariant)
			"""
			...
		@property
		def intersections(self) -> Intersections.Nothing | Intersections.UniformPoints | Intersections.EndsAndMiddle:
			...
		@property
		def lines(self) -> Lines.Monocolor | Lines.Gradient | Lines.SegmentColors:
			...
		def with_intersections(self, intersections: Intersections.Nothing | Intersections.UniformPoints | Intersections.EndsAndMiddle) -> GridPatternOptions.Uniform:
			...
		def with_lines(self, lines: Lines.Monocolor | Lines.Gradient | Lines.SegmentColors) -> GridPatternOptions.Uniform:
			...
		...
class EndPoint:
	class BorderedMatch(object):
		"""
		Draw a point that matches the starting/ending color with a border
		"""
		def __init__(self, match_radius: float, border: Marker) -> None:
			"""
			Draw a point that matches the starting/ending color with a border
			"""
			...
		@property
		def match_radius(self) -> float:
			...
		@property
		def border(self) -> Marker:
			...
		def with_match_radius(self, match_radius: float) -> EndPoint.BorderedMatch:
			...
		def with_border(self, border: Marker) -> EndPoint.BorderedMatch:
			...
		...
	class Match(object):
		"""
		Draw a point that matches the starting/ending color
		"""
		def __init__(self, radius: float) -> None:
			"""
			Draw a point that matches the starting/ending color
			"""
			...
		@property
		def radius(self) -> float:
			...
		def with_radius(self, radius: float) -> EndPoint.Match:
			...
		...
	class Point(object):
		"""
		Draw a normal point
		"""
		def __init__(self, point: Point.None_ | Point.Single | Point.Double) -> None:
			"""
			Draw a normal point
			"""
			...
		@property
		def point(self) -> Point.None_ | Point.Single | Point.Double:
			...
		def with_point(self, point: Point.None_ | Point.Single | Point.Double) -> EndPoint.Point:
			...
		...
class SquareGrid(Grid):
	"""
	Grid of fixed sized tiles where the patterns are automatically scaled to fit within.
	"""
	def __init__(self, patterns: list[PatternVariant], max_width: int, max_scale: float, x_pad: float, y_pad: float) -> None:
		"""
		Creats a grid of fixed size tiles where the patterns are automatically scaled to fit within.
		 :param patterns: Array of patterns to fit on to the grid
		 :param max_width: Maximum number of tiles to lay down horizontally before wrapping around
		 :param max_scale: Maximum scale of patterns in each tile (1 is no limit)
		 :param x_pad: amount of horizontal padding between tiles (measured in scale*x_pad pixels) 
		 :param y_pad: amount of vertical padding between tiles (measured in scale*y_pad pixels) 
		"""
		...
	...
class HexGrid(Grid):
	"""
	A hexagonal grid where patterns are all rendered to fit on the grid.
	"""
	def __init__(self, patterns: list[PatternVariant], max_width: int) -> None:
		"""
		Creates a hexagonal grid where patterns are all rendered to fit on the grid.
		 :param patterns: Array of patterns to fit on to the grid
		 :param max_width: The maximum width of the grid (in grid points) before wrapping around
		"""
		...
	...
class Grid(object):
	"""
	Grid parent class for rendering grids
	 Current grids implemented: HexGrid, SquareGrid
	"""
	def draw_png(self, scale: float, options: GridOptions, padding: None | float = None) -> list[int]:
		"""
		Draws the grid and returns a PNG as an array of bytes
		 :param scale: (HexGrid) distance between points in pixels, (SquareGrid) size of tiles
		 :param options: The options for how to draw the patterns
		 :param padding: Optional padding to put around the grid
		"""
		...
	def draw_to_file(self, file_name: str, scale: float, options: GridOptions, padding: None | float = None) -> None:
		"""
		Draws the grid and saves it to a file
		 :param file_name: path to the file you want to save it as
		 :param scale: (HexGrid) distance between points in pixels, (SquareGrid) size of tiles
		 :param options: The options for how to draw the patterns
		 :param padding: Optional padding to put around the grid
		"""
		...
	def get_bound_scale(self, bound: tuple[float, float], options: float | GridOptions) -> float:
		"""
		Gets the max scale that will fit within the given image size
		 :param bound: x and y maximum sizes
		 :param options: The size of padding or the GridOptions to determine it automatically
		"""
		...
	...
class CollisionOption:
	class OverloadedParallel(object):
		"""
		Same as CollisionOption.ParallelLines except with an escape when you get too many overlaps
		"""
		def __init__(self, max_line: int, overload: OverloadOptions.Dashes | OverloadOptions.LabeledDashes | OverloadOptions.MatchedDashes) -> None:
			"""
			Same as CollisionOption.ParallelLines except with an escape when you get too many overlaps
			:param max_line: number of overlapping segments/lines before using the overload option
			:param overload: Rendering option for when reaching too many parallel lines
			"""
			...
		@property
		def max_line(self) -> int:
			"""
			number of overlapping segments/lines before using the overload option
			"""
			...
		@property
		def overload(self) -> OverloadOptions.Dashes | OverloadOptions.LabeledDashes | OverloadOptions.MatchedDashes:
			"""
			Rendering option for when reaching too many parallel lines
			"""
			...
		def with_max_line(self, max_line: int) -> CollisionOption.OverloadedParallel:
			...
		def with_overload(self, overload: OverloadOptions.Dashes | OverloadOptions.LabeledDashes | OverloadOptions.MatchedDashes) -> CollisionOption.OverloadedParallel:
			...
		...
	class ParallelLines(object):
		"""
		Draws each of the segments as smaller, parallel lines all next to eachother
		"""
		def __init__(self) -> None:
			"""
			Draws each of the segments as smaller, parallel lines all next to eachother
			"""
			...
		...
	class MatchedDashes(object):
		"""
		Draws the line as a set of dashes where the dash marks match the colors of the overlapping lines
		"""
		def __init__(self) -> None:
			"""
			Draws the line as a set of dashes where the dash marks match the colors of the overlapping lines
			"""
			...
		...
	class Dashes(object):
		"""
		Draws the first segment and then dashes of the given color for the rest
		"""
		def __init__(self, color: Color) -> None:
			"""
			Draws the first segment and then dashes of the given color for the rest
			:param color: Color of dashes to draw with
			"""
			...
		@property
		def color(self) -> Color:
			"""
			Color of dashes to draw with
			"""
			...
		def with_color(self, color: Color) -> CollisionOption.Dashes:
			...
		...
class Intersections:
	class EndsAndMiddle(object):
		"""
		Draws a different point for the start, end, and middle
		"""
		def __init__(self, start: EndPoint.Point | EndPoint.Match | EndPoint.BorderedMatch, middle: Point.None_ | Point.Single | Point.Double, end: EndPoint.Point | EndPoint.Match | EndPoint.BorderedMatch) -> None:
			"""
			Draws a different point for the start, end, and middle
			"""
			...
		@property
		def start(self) -> EndPoint.Point | EndPoint.Match | EndPoint.BorderedMatch:
			...
		@property
		def middle(self) -> Point.None_ | Point.Single | Point.Double:
			...
		@property
		def end(self) -> EndPoint.Point | EndPoint.Match | EndPoint.BorderedMatch:
			...
		def with_start(self, start: EndPoint.Point | EndPoint.Match | EndPoint.BorderedMatch) -> Intersections.EndsAndMiddle:
			...
		def with_middle(self, middle: Point.None_ | Point.Single | Point.Double) -> Intersections.EndsAndMiddle:
			...
		def with_end(self, end: EndPoint.Point | EndPoint.Match | EndPoint.BorderedMatch) -> Intersections.EndsAndMiddle:
			...
		...
	class UniformPoints(object):
		"""
		Draws the same point for everything, including start and end
		"""
		def __init__(self, point: Point.None_ | Point.Single | Point.Double) -> None:
			"""
			Draws the same point for everything, including start and end
			"""
			...
		@property
		def point(self) -> Point.None_ | Point.Single | Point.Double:
			...
		def with_point(self, point: Point.None_ | Point.Single | Point.Double) -> Intersections.UniformPoints:
			...
		...
	class Nothing(object):
		"""
		Doesn't draw any points
		"""
		def __init__(self) -> None:
			"""
			Doesn't draw any points
			"""
			...
		...
class AngleSig(object):
	"""
	Angle sigs of a pattern (ex. qqq)
	"""
	def __init__(self, sigs: str) -> None:
		"""
		Make a new angle sig
		 :param sigs: String of sigs (ex. qqq)
		"""
		...
	def get_sigs(self) -> str:
		"""
		 gets the sigs as a string
		"""
		...
	def __repr__(self) -> str:
		...
	...
class Point:
	class Double(object):
		"""
		Draws an inner dot dotand outer dot (or a point with a border)
		"""
		def __init__(self, inner: Marker, outer: Marker) -> None:
			"""
			Draws an inner dot dotand outer dot (or a point with a border)
			:param inner: Marker specifying radius and color of the inner point
			:param outer: Marker specifying radius and color of the outer point
			"""
			...
		@property
		def inner(self) -> Marker:
			"""
			Marker specifying radius and color of the inner point
			"""
			...
		@property
		def outer(self) -> Marker:
			"""
			Marker specifying radius and color of the outer point
			"""
			...
		def with_inner(self, inner: Marker) -> Point.Double:
			...
		def with_outer(self, outer: Marker) -> Point.Double:
			...
		...
	class Single(object):
		"""
		Draws a single dot
		"""
		def __init__(self, marker: Marker) -> None:
			"""
			Draws a single dot
			:param marker: Marker specifying radius and color of point
			"""
			...
		@property
		def marker(self) -> Marker:
			"""
			Marker specifying radius and color of point
			"""
			...
		def with_marker(self, marker: Marker) -> Point.Single:
			...
		...
	class None_(object):
		"""
		Doesn't draw any points
		"""
		def __init__(self) -> None:
			"""
			Doesn't draw any points
			"""
			...
		...
class Marker(object):
	"""
	Specifier for how to draw a shape (not necessarily a circle)
	"""
	def __init__(self, color: Color, radius: float) -> None:
		"""
		Specifier for how to draw a shape (not necessarily a circle)
		:param color: The color to draw it with
		:param radius: The radius of the shape
		"""
		...
	@property
	def color(self) -> Color:
		"""
		The color to draw it with
		"""
		...
	@property
	def radius(self) -> float:
		"""
		The radius of the shape
		"""
		...
	def with_color(self, color: Color) -> Marker:
		...
	def with_radius(self, radius: float) -> Marker:
		...
	...
class Lines:
	class SegmentColors(object):
		"""
		Changes colors whenever it reaches an intersection that's already had the current color
		"""
		def __init__(self, colors: list[Color], triangles: Triangle.None_ | Triangle.Match | Triangle.BorderMatch | Triangle.BorderStartMatch, collisions: CollisionOption.Dashes | CollisionOption.MatchedDashes | CollisionOption.ParallelLines | CollisionOption.OverloadedParallel) -> None:
			"""
			Changes colors whenever it reaches an intersection that's already had the current color
			:param colors: Colors to use
			:param triangles: Arrows/Triangles to draw at the start and when switching between colors
			:param collisions: Options for impossible patterns (when you get overlapping segments)
			"""
			...
		@property
		def colors(self) -> list[Color]:
			"""
			Colors to use
			"""
			...
		@property
		def triangles(self) -> Triangle.None_ | Triangle.Match | Triangle.BorderMatch | Triangle.BorderStartMatch:
			"""
			Arrows/Triangles to draw at the start and when switching between colors
			"""
			...
		@property
		def collisions(self) -> CollisionOption.Dashes | CollisionOption.MatchedDashes | CollisionOption.ParallelLines | CollisionOption.OverloadedParallel:
			"""
			Options for impossible patterns (when you get overlapping segments)
			"""
			...
		def with_colors(self, colors: list[Color]) -> Lines.SegmentColors:
			...
		def with_triangles(self, triangles: Triangle.None_ | Triangle.Match | Triangle.BorderMatch | Triangle.BorderStartMatch) -> Lines.SegmentColors:
			...
		def with_collisions(self, collisions: CollisionOption.Dashes | CollisionOption.MatchedDashes | CollisionOption.ParallelLines | CollisionOption.OverloadedParallel) -> Lines.SegmentColors:
			...
		...
	class Gradient(object):
		"""
		Gradient slowly switches between colors (gradient)
		"""
		def __init__(self, colors: list[Color], segments_per_color: int, bent: bool) -> None:
			"""
			Gradient slowly switches between colors (gradient)
			:param colors: Vec of colors to draw gradients between
			:param segments_per_color: Minimum number of segments before adding another color to switch between
			:param bent: Whether or not to have the segments bend around corners
			"""
			...
		@property
		def colors(self) -> list[Color]:
			"""
			Vec of colors to draw gradients between
			 If the vec only has 1 item, it's treated as Monocolor
			"""
			...
		@property
		def segments_per_color(self) -> int:
			"""
			Minimum number of segments before adding another color to switch between
			 Eg. if segments_per_color = 10,
			 1-9 segments - maximum of 2 colors
			 10-19 segments - maximum of 3 colors, 
			"""
			...
		@property
		def bent(self) -> bool:
			"""
			Whether or not to have the segments bend around corners
			"""
			...
		def with_colors(self, colors: list[Color]) -> Lines.Gradient:
			...
		def with_segments_per_color(self, segments_per_color: int) -> Lines.Gradient:
			...
		def with_bent(self, bent: bool) -> Lines.Gradient:
			...
		...
	class Monocolor(object):
		"""
		Monocolor draws the lines in a single color
		 if bent = true, the corners will bend on the intersections
		"""
		def __init__(self, color: Color, bent: bool) -> None:
			"""
			Monocolor draws the lines in a single color
			 if bent = true, the corners will bend on the intersections
			:param color: Color to draw the lines with
			:param bent: Whether or not it bends at intersection points
			"""
			...
		@property
		def color(self) -> Color:
			"""
			Color to draw the lines with
			"""
			...
		@property
		def bent(self) -> bool:
			"""
			Whether or not it bends at intersection points
			"""
			...
		def with_color(self, color: Color) -> Lines.Monocolor:
			...
		def with_bent(self, bent: bool) -> Lines.Monocolor:
			...
		...
class Triangle:
	class BorderStartMatch(object):
		"""
		Same as Triangle.BorderMatch except with an extra triangle right after the start point
		"""
		def __init__(self, match_radius: float, border: Marker) -> None:
			"""
			Same as Triangle.BorderMatch except with an extra triangle right after the start point
			:param match_radius: radius of how big the match triangle is (as a percentage of segment length)
			:param border: a Marker for the border
			"""
			...
		@property
		def match_radius(self) -> float:
			"""
			radius of how big the match triangle is (as a percentage of segment length)
			"""
			...
		@property
		def border(self) -> Marker:
			"""
			a Marker for the border
			"""
			...
		def with_match_radius(self, match_radius: float) -> Triangle.BorderStartMatch:
			...
		def with_border(self, border: Marker) -> Triangle.BorderStartMatch:
			...
		...
	class BorderMatch(object):
		"""
		Same as Triangle.Match except with an extra border around it
		"""
		def __init__(self, match_radius: float, border: Marker) -> None:
			"""
			Same as Triangle.Match except with an extra border around it
			:param match_radius: radius of how big the match triangle is (as a percentage of segment length)
			:param border: a Marker for the border
			"""
			...
		@property
		def match_radius(self) -> float:
			"""
			radius of how big the match triangle is (as a percentage of segment length)
			"""
			...
		@property
		def border(self) -> Marker:
			"""
			a Marker for the border
			"""
			...
		def with_match_radius(self, match_radius: float) -> Triangle.BorderMatch:
			...
		def with_border(self, border: Marker) -> Triangle.BorderMatch:
			...
		...
	class Match(object):
		"""
		Match the color of the line
		"""
		def __init__(self, radius: float) -> None:
			"""
			Match the color of the line
			:param radius: radius is how big it is (as a percentage of segment length)
			"""
			...
		@property
		def radius(self) -> float:
			"""
			radius is how big it is (as a percentage of segment length)
			"""
			...
		def with_radius(self, radius: float) -> Triangle.Match:
			...
		...
	class None_(object):
		"""
		None, simply don't draw them
		"""
		def __init__(self) -> None:
			"""
			None, simply don't draw them
			"""
			...
		...
class OverloadOptions:
	class MatchedDashes(object):
		"""
		same as CollisionOption,MatchedDashes (represents them as dashes that represet each color of overlapping lines)
		"""
		def __init__(self) -> None:
			"""
			same as CollisionOption,MatchedDashes (represents them as dashes that represet each color of overlapping lines)
			"""
			...
		...
	class LabeledDashes(object):
		"""
		Similar to OverloadOptions.Dashes except it includes a label with the number of overlapping lines
		"""
		def __init__(self, color: Color, label: Marker) -> None:
			"""
			Similar to OverloadOptions.Dashes except it includes a label with the number of overlapping lines
			:param color: Color to draw the dashes with
			:param label: marker for size and color to draw the label
			"""
			...
		@property
		def color(self) -> Color:
			"""
			Color to draw the dashes with
			"""
			...
		@property
		def label(self) -> Marker:
			"""
			marker for size and color to draw the label
			"""
			...
		def with_color(self, color: Color) -> OverloadOptions.LabeledDashes:
			...
		def with_label(self, label: Marker) -> OverloadOptions.LabeledDashes:
			...
		...
	class Dashes(object):
		"""
		same as CollisionOption.Dashes (just draws dashes of the given color over the first line)
		"""
		def __init__(self, color: Color) -> None:
			"""
			same as CollisionOption.Dashes (just draws dashes of the given color over the first line)
			"""
			...
		@property
		def color(self) -> Color:
			...
		def with_color(self, color: Color) -> OverloadOptions.Dashes:
			...
		...
class Color(object):
	"""
	Color struct, using RGBA
	"""
	def __init__(self, r: int, g: int, b: int, a: int) -> None:
		"""
		Color struct, using RGBA
		:param r: Amount of red (0-255)
		:param g: Amount of red (0-255)
		:param b: Amount of red (0-255)
		:param a: Amount of red (0-255)
		"""
		...
	@property
	def r(self) -> int:
		"""
		Amount of red (0-255)
		"""
		...
	@property
	def g(self) -> int:
		"""
		Amount of red (0-255)
		"""
		...
	@property
	def b(self) -> int:
		"""
		Amount of red (0-255)
		"""
		...
	@property
	def a(self) -> int:
		"""
		Amount of red (0-255)
		"""
		...
	def with_r(self, r: int) -> Color:
		...
	def with_g(self, g: int) -> Color:
		...
	def with_b(self, b: int) -> Color:
		...
	def with_a(self, a: int) -> Color:
		...
	...