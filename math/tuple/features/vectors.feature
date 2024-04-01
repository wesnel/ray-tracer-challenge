Feature: Vectors

  Scenario: vector() creates tuples with w=0
    Given v <- vector(4, -4, 3)
    Then v = tuple(4, -4, 3, 0)

  Scenario: Subtracting two vectors
    Given v1 <- vector(3, 2, 1)
      And v2 <- vector(5, 6, 7)
    Then v1 - v2 = vector(-2, -4, -6)

  Scenario: Adding two vectors
    Given a1 <- vector(3, -2, 5)
      And a2 <- vector(-2, 3, 1)
    Then a1 + a2 = vector(1, 1, 6)

  Scenario: Negating a vector
    Given a <- vector(1, -2, 3)
    Then -a = vector(-1, 2, -3)

  Scenario: Multiplying a vector by a scalar
    Given a <- vector(1, -2, 3)
    Then a * 3.5 = vector(3.5, -7, 10.5)

  Scenario: Multiplying a vector by a fraction
    Given a <- vector(1, -2, 3)
    Then a * 0.5 = vector(0.5, -1, 1.5)

  Scenario: Dividing a vector by a scalar
    Given a <- vector(1, -2, 3)
    Then a / 2 = vector(0.5, -1, 1.5)

  Scenario: Subtracting a vector from the zero vector
    Given zero <- vector(0, 0, 0)
      And v <- vector(1, -2, 3)
    Then zero - v = vector(-1, 2, -3)

  Scenario: Computing the magnitude of vector(1, 0, 0)
    Given v <- vector(1, 0, 0)
    Then magnitude(v) = 1

  Scenario: Computing the magnitude of vector(0, 1, 0)
    Given v <- vector(0, 1, 0)
    Then magnitude(v) = 1

  Scenario: Computing the magnitude of vector(0, 0, 1)
    Given v <- vector(0, 0, 1)
    Then magnitude(v) = 1

  Scenario: Computing the magnitude of vector(1, 2, 3)
    Given v <- vector(1, 2, 3)
    Then magnitude(v) = 3.74166

  Scenario: Computing the magnitude of vector(-1, -2, -3)
    Given v <- vector(-1, -2, -3)
    Then magnitude(v) = 3.74166

  Scenario: Normalizing vector(4, 0, 0) gives (1, 0, 0)
    Given v <- vector(4, 0, 0)
    Then normalize(v) = vector(1, 0, 0)

  Scenario: Normalizing vector(1, 2, 3)
    Given v <- vector(1, 2, 3)
    Then normalize(v) = vector(0.26726, 0.53452, 0.80178)

  Scenario: The magnitude of a normalized vector
    Given v <- vector(1, 2, 3)
    When norm <- normalize(v)
    Then magnitude(norm) = 1

  Scenario: The dot product of two vectors
    Given a <- vector(1, 2, 3)
      And b <- vector(2, 3, 4)
    Then dot(a, b) = 20

  Scenario: The cross product of two vectors
    Given a <- vector(1, 2, 3)
      And b <- vector(2, 3, 4)
    Then cross(a, b) = vector(-1, 2, -1)
     And cross(b, a) = vector(1, -2, 1)
