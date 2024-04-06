
from .hex_renderer_python import *

__doc__ = hex_renderer_python.__doc__
if hasattr(hex_renderer_python, "__all__"):
    __all__ = hex_renderer_python.__all__

GridPatternOptions.AnyGridPatternOptions = GridPatternOptions.Uniform | GridPatternOptions.Changing
Intersections.AnyIntersections = Intersections.Nothing | Intersections.UniformPoints | Intersections.EndsAndMiddle
CollisionOption.AnyCollisionOption = CollisionOption.Dashes | CollisionOption.MatchedDashes | CollisionOption.ParallelLines | CollisionOption.OverloadedParallel
Triangle.AnyTriangle = Triangle.None_ | Triangle.Match | Triangle.BorderMatch | Triangle.BorderStartMatch
OverloadOptions.AnyOverloadOptions = OverloadOptions.Dashes | OverloadOptions.LabeledDashes | OverloadOptions.MatchedDashes
ModuleTestTwo.ModuleTest.hello = ModuleTestTwo.ModuleTest.ItemOne | ModuleTestTwo.ModuleTest.ItemTwo | ModuleTestTwo.ModuleTest.ItemThree
EndPoint.AnyEndPoint = EndPoint.Point | EndPoint.Match | EndPoint.BorderedMatch
Lines.AnyLines = Lines.Monocolor | Lines.Gradient | Lines.SegmentColors
Point.AnyPoint = Point.None_ | Point.Single | Point.Double