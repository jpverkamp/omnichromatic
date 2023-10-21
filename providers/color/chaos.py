import random
import sys

used = set()

while True:
    command = sys.stdin.readline()

    if command.startswith('get-color'):
        while True:
            r = random.randint(0, 255)
            g = random.randint(0, 255)
            b = random.randint(0, 255)

            if (r, g, b) not in used:
                break

        used.add((r, g, b))
        sys.stdout.write(f'color {r} {g} {b}\n')
        sys.stdout.flush()