import os
import numpy as np
import time

height = 40
width = 20
# Change BEMCHMARK to True and iterations to 50000 test the performance
iterations = 100
BENCHMARK = False

startState = np.zeros((width, height), dtype=bool)


def updateState(oldState, newState):
    for row in range(0, len(oldState)):
        for col in range(0, len(oldState[row])):
            count = countNeighbors(oldState, row, col)
            newState[row][col] = count == 3 or (
                oldState[row][col] and count == 2)
    return newState


def countNeighbors(state, x, y):
    count = 0
    for row in range(-1, 2):
        for col in range(-1, 2):
            if row == 0 and col == 0:
                continue
            if readState(state, x + row, y + col):
                count = count + 1
    return count


def readState(state, x, y):
    if x < 0:
        x = x + len(state)
    if y < 0:
        y = y + len(state[0])
    if x >= len(state):
        x = 0
    if y >= len(state[0]):
        y = 0

    return state[x][y]


def printState(state):
    if BENCHMARK:
        return
    os.system('clear')
    time.sleep(0.05)
    for row in state:
        buf = ""
        for cell in row:
            if cell:
                buf = buf + "*"
            else:
                buf = buf + "."
        print(buf)


def runSimulation(oldState, steps):
    newState = np.zeros((width, height), dtype=bool)
    for i in range(0, steps):
        newState = updateState(oldState, newState)
        printState(newState)
        tmp = oldState
        oldState = newState
        newState = tmp
    return newState


def main():
    # Splicing the array to input automata seeds
    startState[1][2] = True
    startState[2][3] = True
    startState[3][1:4] = True
    # startState[2][7:10] = True
    printState(startState)
    runSimulation(startState, iterations)


main()
