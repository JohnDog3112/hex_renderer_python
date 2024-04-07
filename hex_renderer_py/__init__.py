
from .hex_renderer_py import *

__doc__ = hex_renderer_py.__doc__
if hasattr(hex_renderer_py, "__all__"):
    __all__ = hex_renderer_py.__all__

Lines.AnyLines = Lines.Monocolor | Lines.Gradient | Lines.SegmentColors
Point.AnyPoint = Point.None_ | Point.Single | Point.Double
EndPoint.AnyEndPoint = EndPoint.Point | EndPoint.Match | EndPoint.BorderedMatch
GridPatternOptions.AnyGridPatternOptions = GridPatternOptions.Uniform | GridPatternOptions.Changing
Triangle.AnyTriangle = Triangle.None_ | Triangle.Match | Triangle.BorderMatch | Triangle.BorderStartMatch
Intersections.AnyIntersections = Intersections.Nothing | Intersections.UniformPoints | Intersections.EndsAndMiddle
OverloadOptions.AnyOverloadOptions = OverloadOptions.Dashes | OverloadOptions.LabeledDashes | OverloadOptions.MatchedDashes
CollisionOption.AnyCollisionOption = CollisionOption.Dashes | CollisionOption.MatchedDashes | CollisionOption.ParallelLines | CollisionOption.OverloadedParallel