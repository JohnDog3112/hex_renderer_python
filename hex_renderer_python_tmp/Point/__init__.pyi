#class Point:

from hex_renderer_python import Marker
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
    def with_inner(self, inner: Marker) -> Double:
        ...
    def with_outer(self, outer: Marker) -> Double:
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
    def with_marker(self, marker: Marker) -> Single:
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