import random
import sys

x = 0
y = 0

while True:
    command = sys.stdin.readline()
    
    if command.startswith('set-size'):
        _, width, height = command.strip().split()
        width = int(width)
        height = int(height)

    elif command.startswith('get-point'):
        response = f'point {x} {y}\n'

        x += 1
        if x >= width:
            x = 0
            y += 1

        sys.stdout.write(response)
        sys.stdout.flush()