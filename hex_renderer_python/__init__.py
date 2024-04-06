
from .hex_renderer_python import *

__doc__ = hex_renderer_python.__doc__
if hasattr(hex_renderer_python, "__all__"):
    __all__ = hex_renderer_python.__all__

OverloadOptions.AnyOverloadOptions = OverloadOptions.Dashes | OverloadOptions.LabeledDashes | OverloadOptions.MatchedDashes
Triangle.AnyTriangle = Triangle.None_ | Triangle.Match | Triangle.BorderMatch | Triangle.BorderStartMatch
GridPatternOptions.AnyGridPatternOptions = GridPatternOptions.Uniform | GridPatternOptions.Changing
Lines.AnyLines = Lines.Monocolor | Lines.Gradient | Lines.SegmentColors
Point.AnyPoint = Point.None_ | Point.Single | Point.Double
Intersections.AnyIntersections = Intersections.Nothing | Intersections.UniformPoints | Intersections.EndsAndMiddle
EndPoint.AnyEndPoint = EndPoint.Point | EndPoint.Match | EndPoint.BorderedMatch
CollisionOption.AnyCollisionOption = CollisionOption.Dashes | CollisionOption.MatchedDashes | CollisionOption.ParallelLines | CollisionOption.OverloadedParallel