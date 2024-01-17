def asteroidCollision(asteroids: list[int]) -> list[int]:
    i = 0
    while i < len(asteroids):
        if asteroids[i] > 0: # Moves right
            if  i+1 < len(asteroids) and asteroids[i+1] < 0: # Collision
                curr_magnitude = abs(asteroids[i])
                next_magnitude = abs(asteroids[i+1])
                if curr_magnitude >= next_magnitude:
                    if curr_magnitude == next_magnitude: # same size
                        asteroids.pop(i)
                        asteroids.pop(i)
                    else: # current > next
                        asteroids.pop(i+1)
                else: # current < next
                        asteroids.pop(i)
                        if i-1 >= 0 and asteroids[i-1] > 0: # previous was positive, go back to the left
                            i-=1
                        else: # previous was negative or not existent, continue to the right
                            i+=1
                continue
        i+=1
    return asteroids

asteroids = [1,1,-1,-2]
print(asteroidCollision(asteroids))
