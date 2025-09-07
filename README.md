# Optimization Algorithms

## Steepest Descent

1- We need starting solution x^t. Zeroise the iteration which is t. Specify tolerance value as ε.
2- at x^t point calculate g^t gradient and ||g^t|| then if ||g^t|| <= ε stop it, else continue.
3- Specfify road direction as d^t = -g^t.
4- Calculate f(x^t + a^t*d^t) as like a^t (step size) is minimum.
5- Calculate new solution point based on: x^(t+1) = x^t + a^t*d^t.
6- Increase iteration counter by 1 and go to 2.step.