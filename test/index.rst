.. hex_renderer_python documentation master file, created by
   sphinx-quickstart on Wed Apr  3 14:04:59 2024.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Welcome to hex_renderer_python's documentation!
===============================================

.. toctree::
   :maxdepth: 2
   :caption: Contents:



Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`

.. py:class:: AngleSig(object)

  Angle sigs of a pattern (ex. qqq)

  .. py:method:: __init__(self, sigs: str)
  
    Make a new angle sig
    :param sigs: String of sigs (ex. qqq)
  
  .. py:method:: get_sigs(self)
  
    gets the sigs as a string
  
  .. py:method:: __repr__(self)
  
  

.. py:module:: OverloadOptions

  Options for what to do when you get too many parallel lines

  .. py:class:: MatchedDashes(object)
  
    same as CollisionOption,MatchedDashes (represents them as dashes that represet each color of overlapping lines)
  
    .. py:method:: __init__(self)
    
      same as CollisionOption,MatchedDashes (represents them as dashes that represet each color of overlapping lines)
    
  
  .. py:class:: LabeledDashes(object)
  
    Similar to OverloadOptions.Dashes except it includes a label with the number of overlapping lines
  
    .. py:method:: __init__(self, color: Color, label: Marker)
    
      Similar to OverloadOptions.Dashes except it includes a label with the number of overlapping lines
      :param color: Color to draw the dashes with
      :param label: marker for size and color to draw the label
    
    .. py:method:: get_color(self)
    
      Color to draw the dashes with
    
    .. py:method:: get_label(self)
    
      marker for size and color to draw the label
    
    .. py:method:: with_color(self, color: Color)
    
    
    .. py:method:: with_label(self, label: Marker)
    
    
  
  .. py:class:: Dashes(object)
  
    same as CollisionOption.Dashes (just draws dashes of the given color over the first line)
  
    .. py:method:: __init__(self, color: Color)
    
      same as CollisionOption.Dashes (just draws dashes of the given color over the first line)
    
    .. py:method:: get_color(self)
    
    
    .. py:method:: with_color(self, color: Color)
    
    
  

.. py:class:: PatternVariant(object)

  A hexpattern that can be rendered on a grid

  .. py:method:: __init__(self, direction: str, angle_sigs: str, great_spell: None | bool)
  
    Creates a new PatternVariant
    :param direction: Starting direction (North_East, East, South_East, South_West, West, North_West)
    :param angle_sigs: String of angle sigs (accepted characters: [q,w,e,d,s,a])
    :param great_spell: Whether or not it's a great spell (Default = false)
  
  .. py:method:: get_direction(self)
  
    Gets the starting direction of the pattern
  
  .. py:method:: get_angle_sigs(self)
  
    Gets the angle_sigs of the pattern
  
  .. py:method:: get_great_spell(self)
  
    Gets whether or not the pattern is a great spell
  

.. py:module:: Triangle

  Options for drawing the triangle/arrow between color changes on the Segment Renderer

  .. py:class:: BorderStartMatch(object)
  
    Same as Triangle.BorderMatch except with an extra triangle right after the start point
  
    .. py:method:: __init__(self, match_radius: float, border: Marker)
    
      Same as Triangle.BorderMatch except with an extra triangle right after the start point
      :param match_radius: radius of how big the match triangle is (as a percentage of segment length)
      :param border: a Marker for the border
    
    .. py:method:: get_match_radius(self)
    
      radius of how big the match triangle is (as a percentage of segment length)
    
    .. py:method:: get_border(self)
    
      a Marker for the border
    
    .. py:method:: with_match_radius(self, match_radius: float)
    
    
    .. py:method:: with_border(self, border: Marker)
    
    
  
  .. py:class:: BorderMatch(object)
  
    Same as Triangle.Match except with an extra border around it
  
    .. py:method:: __init__(self, match_radius: float, border: Marker)
    
      Same as Triangle.Match except with an extra border around it
      :param match_radius: radius of how big the match triangle is (as a percentage of segment length)
      :param border: a Marker for the border
    
    .. py:method:: get_match_radius(self)
    
      radius of how big the match triangle is (as a percentage of segment length)
    
    .. py:method:: get_border(self)
    
      a Marker for the border
    
    .. py:method:: with_match_radius(self, match_radius: float)
    
    
    .. py:method:: with_border(self, border: Marker)
    
    
  
  .. py:class:: Match(object)
  
    Match the color of the line
  
    .. py:method:: __init__(self, radius: float)
    
      Match the color of the line
      :param radius: radius is how big it is (as a percentage of segment length)
    
    .. py:method:: get_radius(self)
    
      radius is how big it is (as a percentage of segment length)
    
    .. py:method:: with_radius(self, radius: float)
    
    
  
  .. py:class:: None_(object)
  
    None, simply don't draw them
  
    .. py:method:: __init__(self)
    
      None, simply don't draw them
    
  

.. py:class:: Marker(object)

  Specifier for how to draw a shape (not necessarily a circle)

  .. py:method:: __init__(self, color: Color, radius: float)
  
    Specifier for how to draw a shape (not necessarily a circle)
    :param color: The color to draw it with
    :param radius: The radius of the shape
  
  .. py:method:: get_color(self)
  
    The color to draw it with
  
  .. py:method:: get_radius(self)
  
    The radius of the shape
  
  .. py:method:: with_color(self, color: Color)
  
  
  .. py:method:: with_radius(self, radius: float)
  
  

.. py:module:: GridPatternOptions

  Struct that holds the different variations of GridPatterns

  .. py:class:: Changing(object)
  
    Changes what pattern renderer to use when finding an introspect or retrospect pattern
    That way you can change colors/renderers for embedded patterns
  
    .. py:method:: __init__(self, variations: list[tuple[Intersections.AnyIntersections, Lines.AnyLines]], intros: list[AngleSig], retros: list[AngleSig])
    
      Changes what pattern renderer to use when finding an introspect or retrospect pattern
      That way you can change colors/renderers for embedded patterns
      :param variations: Variations to use, starts at the first and goes up when it reaches an intro, goes down when reaching a retro
      :param intros: Vec of the angle_sigs of intro patterns
      :param retros: Vec of angle_sigs of retro patterns
    
    .. py:method:: get_variations(self)
    
      Variations to use, starts at the first and goes up when it reaches an intro, goes down when reaching a retro
    
    .. py:method:: get_intros(self)
    
      Vec of the angle_sigs of intro patterns
    
    .. py:method:: get_retros(self)
    
      Vec of angle_sigs of retro patterns
    
    .. py:method:: with_variations(self, variations: list[tuple[Intersections.AnyIntersections, Lines.AnyLines]])
    
    
    .. py:method:: with_intros(self, intros: list[AngleSig])
    
    
    .. py:method:: with_retros(self, retros: list[AngleSig])
    
    
  
  .. py:class:: Uniform(object)
  
    Uniform means that all patterns will be rendered in the same way
    (This excludes the difference with PatternVariant)
  
    .. py:method:: __init__(self, intersections: Intersections.AnyIntersections, lines: Lines.AnyLines)
    
      Uniform means that all patterns will be rendered in the same way
      (This excludes the difference with PatternVariant)
    
    .. py:method:: get_intersections(self)
    
    
    .. py:method:: get_lines(self)
    
    
    .. py:method:: with_intersections(self, intersections: Intersections.AnyIntersections)
    
    
    .. py:method:: with_lines(self, lines: Lines.AnyLines)
    
    
  

.. py:class:: Color(object)

  Color struct, using RGBA

  .. py:method:: __init__(self, r: int, g: int, b: int, a: int)
  
    Color struct, using RGBA
    :param r: Amount of red (0-255)
    :param g: Amount of red (0-255)
    :param b: Amount of red (0-255)
    :param a: Amount of red (0-255)
  
  .. py:method:: get_r(self)
  
    Amount of red (0-255)
  
  .. py:method:: get_g(self)
  
    Amount of red (0-255)
  
  .. py:method:: get_b(self)
  
    Amount of red (0-255)
  
  .. py:method:: get_a(self)
  
    Amount of red (0-255)
  
  .. py:method:: with_r(self, r: int)
  
  
  .. py:method:: with_g(self, g: int)
  
  
  .. py:method:: with_b(self, b: int)
  
  
  .. py:method:: with_a(self, a: int)
  
  

.. py:module:: Lines


  .. py:class:: SegmentColors(object)
  
    Changes colors whenever it reaches an intersection that's already had the current color
  
    .. py:method:: __init__(self, colors: list[Color], triangles: Triangle.AnyTriangle, collisions: CollisionOption.AnyCollisionOption)
    
      Changes colors whenever it reaches an intersection that's already had the current color
      :param colors: Colors to use
      :param triangles: Arrows/Triangles to draw at the start and when switching between colors
      :param collisions: Options for impossible patterns (when you get overlapping segments)
    
    .. py:method:: get_colors(self)
    
      Colors to use
    
    .. py:method:: get_triangles(self)
    
      Arrows/Triangles to draw at the start and when switching between colors
    
    .. py:method:: get_collisions(self)
    
      Options for impossible patterns (when you get overlapping segments)
    
    .. py:method:: with_colors(self, colors: list[Color])
    
    
    .. py:method:: with_triangles(self, triangles: Triangle.AnyTriangle)
    
    
    .. py:method:: with_collisions(self, collisions: CollisionOption.AnyCollisionOption)
    
    
  
  .. py:class:: Gradient(object)
  
    Gradient slowly switches between colors (gradient)
  
    .. py:method:: __init__(self, colors: list[Color], segments_per_color: int, bent: bool)
    
      Gradient slowly switches between colors (gradient)
      :param colors: Vec of colors to draw gradients between
      :param segments_per_color: Minimum number of segments before adding another color to switch between
      :param bent: Whether or not to have the segments bend around corners
    
    .. py:method:: get_colors(self)
    
      Vec of colors to draw gradients between
      If the vec only has 1 item, it's treated as Monocolor
    
    .. py:method:: get_segments_per_color(self)
    
      Minimum number of segments before adding another color to switch between
      Eg. if segments_per_color = 10,
      1-9 segments - maximum of 2 colors
      10-19 segments - maximum of 3 colors,
    
    .. py:method:: get_bent(self)
    
      Whether or not to have the segments bend around corners
    
    .. py:method:: with_colors(self, colors: list[Color])
    
    
    .. py:method:: with_segments_per_color(self, segments_per_color: int)
    
    
    .. py:method:: with_bent(self, bent: bool)
    
    
  
  .. py:class:: Monocolor(object)
  
    Monocolor draws the lines in a single color
    if bent = true, the corners will bend on the intersections
  
    .. py:method:: __init__(self, color: Color, bent: bool)
    
      Monocolor draws the lines in a single color
      if bent = true, the corners will bend on the intersections
      :param color: Color to draw the lines with
      :param bent: Whether or not it bends at intersection points
    
    .. py:method:: get_color(self)
    
      Color to draw the lines with
    
    .. py:method:: get_bent(self)
    
      Whether or not it bends at intersection points
    
    .. py:method:: with_color(self, color: Color)
    
    
    .. py:method:: with_bent(self, bent: bool)
    
    
  

.. py:class:: GridOptions(object)

  Main struct for all pattern rendering options

  .. py:method:: __init__(self, line_thickness: float, pattern_options: GridPatternOptions.AnyGridPatternOptions, center_dot: Point.AnyPoint)
  
    Main struct for all pattern rendering options
    :param line_thickness: Thickness of line in relation to distance between points
    :param pattern_options: Further options for how to render each pattern
    :param center_dot: Optional point to place in the center of each pattern (helps with determining pattern size at a glance)
  
  .. py:method:: get_line_thickness(self)
  
    Thickness of line in relation to distance between points
    eg. if the line_thickness = 0.1, and the distance between points is 10 pixels,
    then the line_thickness would be 1 pixel
  
  .. py:method:: get_pattern_options(self)
  
    Further options for how to render each pattern
  
  .. py:method:: get_center_dot(self)
  
    Optional point to place in the center of each pattern (helps with determining pattern size at a glance)
  
  .. py:method:: with_line_thickness(self, line_thickness: float)
  
  
  .. py:method:: with_pattern_options(self, pattern_options: GridPatternOptions.AnyGridPatternOptions)
  
  
  .. py:method:: with_center_dot(self, center_dot: Point.AnyPoint)
  
  

.. py:module:: Point

  Options for drawing points at the grid points/intersections

  .. py:class:: Double(object)
  
    Draws an inner dot dotand outer dot (or a point with a border)
  
    .. py:method:: __init__(self, inner: Marker, outer: Marker)
    
      Draws an inner dot dotand outer dot (or a point with a border)
      :param inner: Marker specifying radius and color of the inner point
      :param outer: Marker specifying radius and color of the outer point
    
    .. py:method:: get_inner(self)
    
      Marker specifying radius and color of the inner point
    
    .. py:method:: get_outer(self)
    
      Marker specifying radius and color of the outer point
    
    .. py:method:: with_inner(self, inner: Marker)
    
    
    .. py:method:: with_outer(self, outer: Marker)
    
    
  
  .. py:class:: Single(object)
  
    Draws a single dot
  
    .. py:method:: __init__(self, marker: Marker)
    
      Draws a single dot
      :param marker: Marker specifying radius and color of point
    
    .. py:method:: get_marker(self)
    
      Marker specifying radius and color of point
    
    .. py:method:: with_marker(self, marker: Marker)
    
    
  
  .. py:class:: None_(object)
  
    Doesn't draw any points
  
    .. py:method:: __init__(self)
    
      Doesn't draw any points
    
  

.. py:module:: Intersections

  How to draw all the points in a pattern, including start, end, and middle points

  .. py:class:: EndsAndMiddle(object)
  
    Draws a different point for the start, end, and middle
  
    .. py:method:: __init__(self, start: EndPoint.AnyEndPoint, middle: Point.AnyPoint, end: EndPoint.AnyEndPoint)
    
      Draws a different point for the start, end, and middle
    
    .. py:method:: get_start(self)
    
    
    .. py:method:: get_middle(self)
    
    
    .. py:method:: get_end(self)
    
    
    .. py:method:: with_start(self, start: EndPoint.AnyEndPoint)
    
    
    .. py:method:: with_middle(self, middle: Point.AnyPoint)
    
    
    .. py:method:: with_end(self, end: EndPoint.AnyEndPoint)
    
    
  
  .. py:class:: UniformPoints(object)
  
    Draws the same point for everything, including start and end
  
    .. py:method:: __init__(self, point: Point.AnyPoint)
    
      Draws the same point for everything, including start and end
    
    .. py:method:: get_point(self)
    
    
    .. py:method:: with_point(self, point: Point.AnyPoint)
    
    
  
  .. py:class:: Nothing(object)
  
    Doesn't draw any points
  
    .. py:method:: __init__(self)
    
      Doesn't draw any points
    
  

.. py:module:: EndPoint

  Specifier for how to draw the start and end points on a pattern

  .. py:class:: BorderedMatch(object)
  
    Draw a point that matches the starting/ending color with a border
  
    .. py:method:: __init__(self, match_radius: float, border: Marker)
    
      Draw a point that matches the starting/ending color with a border
    
    .. py:method:: get_match_radius(self)
    
    
    .. py:method:: get_border(self)
    
    
    .. py:method:: with_match_radius(self, match_radius: float)
    
    
    .. py:method:: with_border(self, border: Marker)
    
    
  
  .. py:class:: Match(object)
  
    Draw a point that matches the starting/ending color
  
    .. py:method:: __init__(self, radius: float)
    
      Draw a point that matches the starting/ending color
    
    .. py:method:: get_radius(self)
    
    
    .. py:method:: with_radius(self, radius: float)
    
    
  
  .. py:class:: Point(object)
  
    Draw a normal point
  
    .. py:method:: __init__(self, point: Point.AnyPoint)
    
      Draw a normal point
    
    .. py:method:: get_point(self)
    
    
    .. py:method:: with_point(self, point: Point.AnyPoint)
    
    
  

.. py:class:: SquareGrid(Grid)

  Grid of fixed sized tiles where the patterns are automatically scaled to fit within.

  .. py:method:: __init__(self, patterns: list[PatternVariant], max_width: int, max_scale: float, x_pad: float, y_pad: float)
  
    Creats a grid of fixed size tiles where the patterns are automatically scaled to fit within.
    :param patterns: Array of patterns to fit on to the grid
    :param max_width: Maximum number of tiles to lay down horizontally before wrapping around
    :param max_scale: Maximum scale of patterns in each tile (1 is no limit)
    :param x_pad: amount of horizontal padding between tiles (measured in scale*x_pad pixels)
    :param y_pad: amount of vertical padding between tiles (measured in scale*y_pad pixels)
  

.. py:class:: HexGrid(Grid)

  A hexagonal grid where patterns are all rendered to fit on the grid.

  .. py:method:: __init__(self, patterns: list[PatternVariant], max_width: int)
  
    Creates a hexagonal grid where patterns are all rendered to fit on the grid.
    :param patterns: Array of patterns to fit on to the grid
    :param max_width: The maximum width of the grid (in grid points) before wrapping around
  

.. py:class:: Grid(object)

  Grid parent class for rendering grids
  Current grids implemented: HexGrid, SquareGrid

  .. py:method:: draw_png(self, scale: float, options: GridOptions, padding: None | float)
  
    Draws the grid and returns a PNG as an array of bytes
    :param scale: (HexGrid) distance between points in pixels, (SquareGrid) size of tiles
    :param options: The options for how to draw the patterns
    :param padding: Optional padding to put around the grid
  
  .. py:method:: draw_to_file(self, file_name: str, scale: float, options: GridOptions, padding: None | float)
  
    Draws the grid and saves it to a file
    :param file_name: path to the file you want to save it as
    :param scale: (HexGrid) distance between points in pixels, (SquareGrid) size of tiles
    :param options: The options for how to draw the patterns
    :param padding: Optional padding to put around the grid
  
  .. py:method:: get_bound_scale(self, bound: tuple[float, float], options: float | GridOptions)
  
    Gets the max scale that will fit within the given image size
    :param bound: x and y maximum sizes
    :param options: The size of padding or the GridOptions to determine it automatically
  

.. py:module:: CollisionOption

  Options for drawing overlapping segments (impossible patterns)

  .. py:class:: OverloadedParallel(object)
  
    Same as CollisionOption.ParallelLines except with an escape when you get too many overlaps
  
    .. py:method:: __init__(self, max_line: int, overload: OverloadOptions.AnyOverloadOptions)
    
      Same as CollisionOption.ParallelLines except with an escape when you get too many overlaps
      :param max_line: number of overlapping segments/lines before using the overload option
      :param overload: Rendering option for when reaching too many parallel lines
    
    .. py:method:: get_max_line(self)
    
      number of overlapping segments/lines before using the overload option
    
    .. py:method:: get_overload(self)
    
      Rendering option for when reaching too many parallel lines
    
    .. py:method:: with_max_line(self, max_line: int)
    
    
    .. py:method:: with_overload(self, overload: OverloadOptions.AnyOverloadOptions)
    
    
  
  .. py:class:: ParallelLines(object)
  
    Draws each of the segments as smaller, parallel lines all next to eachother
  
    .. py:method:: __init__(self)
    
      Draws each of the segments as smaller, parallel lines all next to eachother
    
  
  .. py:class:: MatchedDashes(object)
  
    Draws the line as a set of dashes where the dash marks match the colors of the overlapping lines
  
    .. py:method:: __init__(self)
    
      Draws the line as a set of dashes where the dash marks match the colors of the overlapping lines
    
  
  .. py:class:: Dashes(object)
  
    Draws the first segment and then dashes of the given color for the rest
  
    .. py:method:: __init__(self, color: Color)
    
      Draws the first segment and then dashes of the given color for the rest
      :param color: Color of dashes to draw with
    
    .. py:method:: get_color(self)
    
      Color of dashes to draw with
    
    .. py:method:: with_color(self, color: Color)