#
#
#
# A practical application – capacity planning with linear programming
#
#
# Let’s assume that we want to maximize the profits of a state-of-the-art factory
# that manufactures two different types of robots:
# • Advanced model (A): This provides full functionality. Manufacturing each unit of the
#   advanced model results in a profit of $4,200.
# • Basic model (B): This only provides basic functionality. Manufacturing each unit of the
#   basic model results in a profit of $2,800.
#
# There are three different types of people needed to manufacture a robot. The exact
# number of days needed to manufacture a robot of each type is as follows:
# Type of Robot              Technician           AI Specialist         Engineer
# Robot A: advanced model      3 days                4 days              4 days
# Robot B: basic model         2 days                3 days              3 days
#
#
# The factory runs on 30-day cycles. A single AI specialist is available for 30 days in a cycle.
# Each of the two engineers will take 8 days off in 30 days. So, an engineer is available only
# for 22 days in a cycle. There is a single technician available for 20 days in a 30-day cycle.
#
# The following table shows the number of people we have in the factory:
#                                   Technician            AI Specialist         Engineer
# Number of people                     1                       1                    2
# Total number of days in a cycle   1 x 20 = 20 days      1 x 30 = 30 days     2 x 22 = 44 days
#
# This can be modeled as follows:
#   • Maximum profit = 4200A + 2800B
#   • This is subject to the following:
#     • A ≥ 0: The number of advanced robots produced can be 0 or more.
#     • B ≥ 0: The number of basic robots produced can be 0 or more.
#     • 3A + 2B ≤ 20: These are the constraints of the technician’s availability.
#     • 4A + 3B ≤ 30: These are the constraints of the AI specialist’s availability.
#     • 4A + 3B ≤ 44 : These are the constraints of the engineers’ availability.

# pulp is used to implement linear programming:
import pulp

# we call the LpProblem function in this package to instantiate the problem class.
# We name the instance Profit maximising problem:
model = pulp.LpProblem("Profit_maximising_problem", pulp.LpMaximize)

# define two linear variables, A and B. Variable A represents the number of advanced robots
# that are produced and variable B represents the number of basic robots that are produced:
A = pulp.LpVariable("A", lowBound=0, cat="Integer")
B = pulp.LpVariable("B", lowBound=0, cat="Integer")


# We define the objective function and constraints as follows:
# Objective function
model += 5000 * A + 2500 * B, "Profit"
# Constraints
model += 3 * A + 2 * B <= 20
model += 4 * A + 3 * B <= 30
model += 4 * A + 3 * B <= 44

# We use the solve function to generate a solution:
# Solve our problem
model.solve()
pulp.LpStatus[model.status]

# Then, we print the values of A and B and the value of the objective function:
# Print our decision variable values
print(A.varValue)  # 6.0
print(B.varValue)  # 1.0

# Print our objective function value
print(pulp.value(model.objective))  # 32500.0
