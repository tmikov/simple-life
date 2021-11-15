import sleepSynchronously from 'sleep-synchronously';

const width = 40;
const height = 20;
const iterations = 100;
const BENCHMARK = false;

let startState = [
    [false, false, false, false, false],
    [false, false, true, false, false],
    [false, false, false, true, false],
    [false, true, true, true, false],
    [false, false, false, false, false],
];

function updateState(oldState, newState) {
    for (let y = 0; y < oldState.length; y = y + 1) {
        for (let x = 0; x < oldState[y].length; x = x + 1) {
            let count = countNeighbors(oldState, x, y);
            newState[y][x] = count === 3 || (oldState[y][x] && count === 2);
        }
    }
}

function printState(state) {
    if (BENCHMARK)
        return;
    clearScreen();
    for (let row of state) {
        let buf = '';
        for (let cell of row) {
            if (cell) {
                buf = buf + '*';
            } else {
                buf = buf + '.';
            }
        }
        console.log(buf);
    }
}

function delay(ms) {
    if (BENCHMARK)
        return;
    sleepSynchronously(ms);
}

function readState(state, x, y) {
    if (y < 0) {
        y = y + state.length
    }
    if (x < 0) {
        x = x + state[0].length
    }
    if (y >= state.length) {
        y = 0
    }
    if (x >= state[0].length) {
        x = 0
    }
    return state[y][x];
}

function countNeighbors(state, x, y) {
    let count = 0;
    for (let row = y - 1; row <= y + 1; row = row + 1) {
        for (let col = x - 1; col <= x + 1; col = col + 1) {
            if (row === y && col === x)
                continue;
            if (readState(state, col, row))
                count = count + 1;

        }
    }
    return count;
}

function createMatrix(w, h) {
    let result = Array(h);
    for (let i = 0; i < h; ++i) {
        result[i] = Array(w).fill(false);
    }
    return result;
}

function clearScreen() {
    process.stdout.write('\u001b[2J\u001b[0;0H');
}

function runSimulation(oldState, steps) {
    let newState = createMatrix(oldState[0].length, oldState.length);
    for (let i = 0; i < steps; i = i + 1) {
        updateState(oldState, newState);

        printState(newState);
        delay(50);

        let tmp = oldState;
        oldState = newState;
        newState = tmp;
    }
    return oldState;
}

function main() {
    // Resize startState.
    if (startState.length < height) {
        for (let i = startState.length; i < height; ++i)
            startState[i] = Array();
        for (let row of startState) {
            for (let i = row.length; i < width; ++i)
                row[i] = false;
        }
    }

    printState(startState);
    runSimulation(startState, iterations);
}

main();
