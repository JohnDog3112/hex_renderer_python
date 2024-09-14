
from .hex_renderer_py import *

__doc__ = hex_renderer_py.__doc__
if hasattr(hex_renderer_py, "__all__"):
    __all__ = hex_renderer_py.__all__

Intersections.AnyIntersections = Intersections.Nothing | Intersections.UniformPoints | Intersections.EndsAndMiddle
Triangle.AnyTriangle = Triangle.None_ | Triangle.Match | Triangle.BorderMatch | Triangle.BorderStartMatch
Lines.AnyLines = Lines.Monocolor | Lines.Gradient | Lines.SegmentColors
EndPoint.AnyEndPoint = EndPoint.Point | EndPoint.Match | EndPoint.BorderedMatch
OverloadOptions.AnyOverloadOptions = OverloadOptions.Dashes | OverloadOptions.LabeledDashes | OverloadOptions.MatchedDashes
GridPatternOptions.AnyGridPatternOptions = GridPatternOptions.Uniform | GridPatternOptions.Changing
CollisionOption.AnyCollisionOption = CollisionOption.Dashes | CollisionOption.MatchedDashes | CollisionOption.ParallelLines | CollisionOption.OverloadedParallel
Point.AnyPoint = Point.None_ | Point.Single | Point.Double