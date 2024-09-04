import math

class Point:
    def __init__(self, x: int | float, y: int | float) -> None:
        self.x = x
        self.y = y
        
    def distance(self):
        return math.sqrt(self.x**2 + self.y**2)
    
    def distance(self, other: "Point"):
        return math.sqrt((self.x - other.x)**2 + (self.y - other.y)**2) 
    
p1 = Point(1, 2)
p2 = Point(3, 4)

# print(f"distance from orgin: {p1.distance()}")
print(f"distance from p2: {p1.distance(p2)}")