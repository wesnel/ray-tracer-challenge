Feature: Points

  Scenario: point() creates tuples with w=1
    Given p <- point(4, -4, 3)
    Then p = tuple(4, -4, 3, 1)

  Scenario: Subtracting two points
    Given p1 <- point(3, 2, 1)
      And p2 <- point(5, 6, 7)
    Then p1 - p2 = vector(-2, -4, -6)

  Scenario: Subtracting a vector from a point
    Given p <- point(3, 2, 1)
      And v <- vector(5, 6, 7)
    Then p - v = point(-2, -4, -6)
