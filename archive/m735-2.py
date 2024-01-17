def asteroidCollision(asteroids: list[int]) -> list[int]:
    stack = []
    for a in asteroids:
        if a > 0:
            stack.append(a)
        else:
            curr_magnitude = abs(a)
            append = True
            while stack and stack[-1] > 0:
                head_magnitude = abs(stack[-1])
                if curr_magnitude >= head_magnitude:
                    stack.pop()
                    if curr_magnitude == head_magnitude:
                        append = False
                        break
                else:
                    append = False
                    break
            if append:
                stack.append(a)

    return stack

asteroids = [1,1,-1,-2]
print(asteroidCollision(asteroids))
