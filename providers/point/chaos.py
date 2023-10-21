import random
import sys

used = set()

while True:
    command = sys.stdin.readline()
    
    if command.startswith('set-size'):
        _, width, height = command.strip().split()
        width = int(width)
        height = int(height)

    elif command.startswith('get-point'):
        while True:
            x = random.randint(0, width - 1)
            y = random.randint(0, height - 1)
            
            if (x, y) not in used:
                break

        used.add((x, y))
        sys.stdout.write(f'point {x} {y}\n')
        sys.stdout.flush()
            