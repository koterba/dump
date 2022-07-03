import pyray as pr
import random

import pi_aprox

# def calculate_pi() -> int:
#     in_square = in_circle = pi = 0
#     while True:
#         x = random.random()
#         y = random.random()

#         dist = (x*x + y*y) ** 0.5
#         in_square += 1

#         if dist <= 1:
#             in_circle += 1
        
#         if in_square % 1000 == 0: ## in square is used only not to waste variable names
#             pi = 4 * in_circle / in_square
#             yield pi


pr.init_window(500, 800, "PI Approximation")
##pr.set_target_fps(60)

circles = []

class Circle:
    def __init__(self, x , y):
        self.x = x
        self.y = y

def add_circle():
    circles.append(Circle(random.randint(0, 500), random.randint(300, 800)))

in_square, in_circle, pi = pi_aprox.get_next_pi(0, 0, 0, 0)

while not pr.window_should_close():
    add_circle()

    pr.begin_drawing()
    pr.clear_background(pr.RAYWHITE)
    pr.draw_circle(250, 550, 250.0, pr.Color(90, 90, 90, 255))
    for circle in circles:
        pr.draw_circle(circle.x, circle.y, 2.0, pr.Color(20, 20, 20, 255))
    
    in_square, in_circle, pi = pi_aprox.get_next_pi(1000000, in_square, in_circle, pi)
    pr.draw_text("PI:", 250 - int(pr.measure_text("PI:", 50) / 2), 10, 50, pr.Color(0, 0, 0, 255))
    ## SUBSTITUTE "10" BELOW FOR "250 - int(pr.measure_text(num, 50) / 2)" TO GET CENTERED TEXT
    pr.draw_text(str(pi), 10, 70, 50, pr.Color(0, 0, 0, 255))
    pr.end_drawing()

pr.close_window()