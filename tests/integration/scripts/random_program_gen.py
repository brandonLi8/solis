# Copyright © 2022-2023 Brandon Li. All rights reserved.

## Python script that was used to generate the `random_*.sol` integration tests.

import random
import struct
import copy
import textwrap

# seeded random for repeatability
random.seed(1)

class Expression:
  """
  A class representing a solis expression, with the program text and the program value.

  -----------
  text: string - the text representation of the expression (in the program)
  value: * - the value of the expression.
  """
  def __init__(self, text, value):
    self.text = text
    self.value = value

class Identifier:
  """
  A class representing a solis identifier.
  """
  def __init__(self, type_ref, value):
    self.type_ref = type_ref
    self.value = value

#!————————————————————————————————————————————————————————————————————————————*!
# Identifier Generators
#!————————————————————————————————————————————————————————————————————————————*!

# Number of identifiers that have been created
id_count = 0

def gen_unique_id():
  """
  Generates a unique id in alphabetic order
  >>> gen_unique_id() // 'a1'
  >>> gen_unique_id() // 'b1'
  ...
  >>> gen_unique_id() // 'a2'
  """
  global id_count

  result = chr(ord('a') + id_count % 24) + str(id_count // 24 + 1)
  id_count += 1
  return result

def reset_id_count():
  global id_count
  id_count = 0

def has_id_of_type(bindings, type_ref):
  """
  Returns if there are any bindings of type type_ref
  - bindings: Map<name, Identifier>
  """
  return any(bindings[binding].type_ref == type_ref for binding in bindings)

#!————————————————————————————————————————————————————————————————————————————*!
# General Expression Generators
#!————————————————————————————————————————————————————————————————————————————*!

def gen_id(bindings, type_ref):
  """
  Generates a random identifier inside bindings, with type type_ref.
  - bindings: Map<name, Identifier>
  """
  result = random.choice([binding for binding in bindings if bindings[binding].type_ref == type_ref])
  return Expression(result, bindings[result].value)

def gen_binding(bindings, type_ref):
  """
  Returns a random binding (`let _: [type_ref] = `)
  """
  id_name = gen_unique_id()
  expression = TYPED_GENERATORS[type_ref](bindings)

  bindings[id_name] = Identifier(type_ref, expression.value)
  return Expression(f"let {id_name}: {type_ref} = {expression.text}", expression.value)

def gen_if(bindings, type_ref):
  """
  Returns an if statement that results in type type_ref
  """
  num_branches = random.randrange(2, 8)
  result_value = None
  text = ''

  for i in range(num_branches):
    is_last_branch = i == num_branches - 1

    if not is_last_branch:
      condition = gen_random_bool(bindings)
      text += f"if {condition.text} "

    text += "{\n"

    block = gen_block(copy.copy(bindings), random.randrange(1, 10), type_ref)

    text += "  " + block.text.replace("\n", "\n  ") + "\n}"
    if not is_last_branch:
      text += " else "

    if result_value is None and (condition.value or is_last_branch):
      result_value = block.value

  return Expression(text, result_value)

#!————————————————————————————————————————————————————————————————————————————*!
# Int Expression Generators
#!————————————————————————————————————————————————————————————————————————————*!

def gen_int_literal(bindings):
  """
  Returns a random int literal expression.
  """
  result = random.randrange(1000)
  return Expression(str(result), result)

def gen_int_binary_exp(bindings):
  """
  Returns a binary expression that results in a int.
  """
  operators = [
    ('+', lambda a, b: a + b),
    ('-', lambda a, b: a - b),
    ('*', lambda a, b: a * b),
    ('/', lambda a, b: int(a / b)),
    ('%', lambda a, b: a - int(a / b) * b)
  ]
  random_int_1 = gen_random_int(bindings)
  random_int_2 = gen_random_int(bindings)

  if random_int_2.value == 0:
    operators = operators[:-2]

  operator = random.choice(operators)
  return Expression(
    f"({random_int_1.text}) {operator[0]} ({random_int_2.text})",
    operator[1](random_int_1.value, random_int_2.value)
  )

def gen_random_int(bindings):
  """
  Returns a randomly generated integer expression.
  """
  global tab
  generators = [gen_int_literal] * 4  + [gen_int_binary_exp]

  if has_id_of_type(bindings, "int"):
    generators.extend([lambda bindings: gen_id(bindings, "int")] * 4)

  generators = generators * 4
  generators.append(lambda bindings: gen_if(bindings, "int"))

  result = random.choice(generators)(bindings)

  if random.choice([True, False]):
    return Expression('-(' + result.text + ')', -result.value)
  return result

#!————————————————————————————————————————————————————————————————————————————*!
# Float Expression Generators
#!————————————————————————————————————————————————————————————————————————————*!

def gen_float_literal(bindings):
  """
  Returns a random float literal expression.
  """
  result = random.random() * 50.123
  return Expression(str(result), result)

def gen_float_binary_exp(bindings):
  """
  Returns a binary expression that results in a float.
  """
  operators = [
    ('+', lambda a, b: a + b),
    ('-', lambda a, b: a - b),
    ('*', lambda a, b: a * b),
    ('/', lambda a, b: a / b),
  ]
  operand_generators = random.choice([
    (gen_random_float, gen_random_float),
    (gen_random_float, gen_random_int),
    (gen_random_int, gen_random_float)
  ])

  random_float_1 = operand_generators[0](bindings)
  random_float_2 = operand_generators[1](bindings)

  if random_float_2.value == 0:
    operators = operators[:-2]

  operator = random.choice(operators)
  return Expression(
    f"({random_float_1.text}) {operator[0]} ({random_float_2.text})",
    operator[1](random_float_1.value, random_float_2.value)
  )

def gen_random_float(bindings):
  """
  Returns a randomly generated float expression.
  """
  generators = [gen_float_literal] * 4 + [gen_float_binary_exp]

  if has_id_of_type(bindings, "float"):
    generators.extend([lambda bindings: gen_id(bindings, "float")] * 4)

  result = random.choice(generators)(bindings)

  generators = generators * 4
  generators.append(lambda bindings: gen_if(bindings, "float"))

  if random.choice([True, False]):
    return Expression('-(' + result.text + ')', -result.value)
  return result

#!————————————————————————————————————————————————————————————————————————————*!
# Bool Generators
#!————————————————————————————————————————————————————————————————————————————*!

def gen_bool_literal(bindings):
  """
  Returns a random bool literal expression.
  """
  return random.choice([Expression("true", True), Expression("false", False)])

def gen_bool_binary_exp(bindings):
  """
  Returns a binary expression that results in a bool.
  """
  operators = [
    ('<', lambda a, b: a < b),
    ('<=', lambda a, b: a <= b),
    ('>', lambda a, b: a > b),
    ('>=', lambda a, b: a >= b),
  ]
  operand_1 = random.choice([gen_random_int, gen_random_float])(bindings)
  operand_2 = random.choice([gen_random_int, gen_random_float])(bindings)
  operator = random.choice(operators)

  return Expression(
    f"({operand_1.text}) {operator[0]} ({operand_2.text})",
    operator[1](operand_1.value, operand_2.value)
  )

def gen_bool_from_equality(bindings):
  operators = [('==', lambda a, b: a == b), ('!=', lambda a, b: a != b)]
  operand_gen = random.choice([gen_random_int, gen_random_bool, gen_random_float])

  operand_1 = operand_gen(bindings)
  operand_2 = operand_gen(bindings)
  operator = random.choice(operators)
  return Expression(
    f"({operand_1.text}) {operator[0]} ({operand_2.text})",
    operator[1](operand_1.value, operand_2.value)
  )

def gen_random_bool(bindings):
  """
  Returns a randomly generated bool expression.
  """
  generators = [gen_bool_literal] * 4 + [gen_bool_binary_exp, gen_bool_from_equality]

  if has_id_of_type(bindings, "bool"):
    generators.extend([lambda bindings: gen_id(bindings, "bool")] * 4)

  generators = generators * 4
  generators.append(lambda bindings: gen_if(bindings, "bool"))

  result = random.choice(generators)(bindings)

  if random.choice([True, False]):
    return Expression('!(' + result.text + ')', not result.value)
  return result

#!————————————————————————————————————————————————————————————————————————————*!
# Test generator
#!————————————————————————————————————————————————————————————————————————————*!

def gen_random_expr(bindings):
  """
  Generates a random expression.
  is_last_expression - true if it is the last expression in a block
  """
  choices = [
    gen_random_int,
    gen_random_float,
    gen_random_bool,
  ] * 3

  choices.extend([lambda bindings: gen_binding(bindings, "int")] * 2)
  choices.extend([lambda bindings: gen_binding(bindings, "float")] * 2)
  choices.extend([lambda bindings: gen_binding(bindings, "bool")] * 2)

  expr = random.choice(choices)(bindings)
  expr.text += ';'
  return expr

def float_to_signed_int(f):
    """
    Prints a float interpreted as a signed int.
    """
    [d] = struct.unpack(">Q", struct.pack(">d", f))
    binary = f'{d:064b}'
    return int(binary[1:], 2) + (0 if binary[0] == '0' else -2 ** 63)

def gen_block(bindings, num_expr, type_ref):
  """
  Generates a block with `num_expr` expressions and results in type `type_ref`
  """
  global tab
  result = []

  for _ in range(num_expr - 1):
    expr = gen_random_expr(bindings)
    result.append(expr)

  # for int and float, the result is a sum of all bindings of that type
  if (type_ref == 'int' and has_id_of_type(bindings, 'int')) or (type_ref == 'float' and has_id_of_type(bindings, 'float')):
    typed_bindings = list(filter(lambda n: bindings[n].type_ref == type_ref, bindings.keys()))
    sampled_bindings = random.sample(typed_bindings, random.randrange(1, len(typed_bindings) + 1));

    if type_ref == 'float' and has_id_of_type(bindings, 'int'):
      int_typed_bindings = list(filter(lambda n: bindings[n].type_ref in 'int', bindings.keys()))
      sampled_bindings += random.sample(typed_bindings, random.randrange(1, len(typed_bindings) + 1));

    result.append(Expression(" + ".join(sampled_bindings), sum(map(lambda n: bindings[n].value, sampled_bindings))))
  else:
    result.append(TYPED_GENERATORS[type_ref](bindings))

  return Expression("\n".join([r.text for r in result]), result[-1].value)

# maps a type_ref to a generator that creates an expression of that type
TYPED_GENERATORS = {
  'int': gen_random_int,
  'float': gen_random_float,
  'bool': gen_random_bool,
}

def gen_program(i):
  """
  Generates random_{i}.py
  """
  reset_id_count()

  bindings = {}
  result_type = random.choice(['int', 'float', 'bool'])
  result = gen_block(bindings, 150, result_type)

  if result_type == 'float':
    result.value = float_to_signed_int(result.value)

  if result_type == 'bool':
    result.value = 0 if result.value == False else 1

  program = textwrap.dedent("""\
    # Copyright © 2022-2023 Brandon Li. All rights reserved.

    ##
    Integration test of a randomly generated program.
    NOTE: this file was auto-generated with the `random_program_gen.py` script.
    ##\n
    """)

  print(f"Generate `random_{i}.sol` with result type `{result_type}`")

  # Write program and result to file
  with open(f'./tests/integration/random_{i}.sol', 'w+') as f:
    f.write(program + result.text)

  # Write expected result to file
  with open(f'./tests/integration/expected/random_{i}.out', 'w+') as f:
    f.write(str(result.value))

for i in range(1, 10):
  gen_program(str(i))
