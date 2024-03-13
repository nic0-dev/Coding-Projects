import sympy as sp

epsilon = sp.symbols('epsilon', real=True, positive=True)
P = sp.Rational(1, 2)
Q0 = (1 - epsilon) / 2
Q1 = (1 + epsilon) / 2
D_KL = P * sp.log(P/Q0, 2) + P * sp.log(P/Q1, 2)
equation = sp.Eq(D_KL, epsilon**2)
solution = sp.solve(equation, epsilon)
print(solution)