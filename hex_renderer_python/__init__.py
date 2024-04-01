
from .hex_renderer_python import *

__doc__ = hex_renderer_python.__doc__
if hasattr(hex_renderer_python, "__all__"):
    __all__ = hex_renderer_python.__all__

GridPatternOptions.AnyGridPatternOptions = GridPatternOptions.Uniform | GridPatternOptions.Changing
EndPoint.AnyEndPoint = EndPoint.Point | EndPoint.Match | EndPoint.BorderedMatch
CollisionOption.AnyCollisionOption = CollisionOption.Dashes | CollisionOption.MatchedDashes | CollisionOption.ParallelLines | CollisionOption.OverloadedParallel
Intersections.AnyIntersections = Intersections.Nothing | Intersections.UniformPoints | Intersections.EndsAndMiddle
Point.AnyPoint = Point.None_ | Point.Single | Point.Double
Lines.AnyLines = Lines.Monocolor | Lines.Gradient | Lines.SegmentColors
Triangle.AnyTriangle = Triangle.None_ | Triangle.Match | Triangle.BorderMatch | Triangle.BorderStartMatch
OverloadOptions.AnyOverloadOptions = OverloadOptions.Dashes | OverloadOptions.LabeledDashes | OverloadOptions.MatchedDashes