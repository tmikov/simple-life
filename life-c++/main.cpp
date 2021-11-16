#include <cassert>
#include <chrono>
#include <cstdio>
#include <thread>
#include <vector>

static constexpr int WIDTH = 40;
static constexpr int HEIGHT = 20;
static constexpr size_t ITERATIONS = 100;
static constexpr bool BENCHMARK = false;

static const char INIT_STATE[] = R"(
..*..
...*.
.***.
.....)";

class Matrix {
    int width_;
    int height_;
    std::vector<bool> data_;

   public:
    explicit Matrix(int width, int height) :
        width_(width), height_(height), data_(width * height, false) {}
    Matrix(Matrix &&) = default;
    Matrix & operator=(Matrix &&) = default;

    auto begin() const { return data_.begin(); }
    auto end() const { return data_.end(); }

    int width() const { return width_; }
    int height() const { return height_; }
    bool read(int x, int y) const {
        assert(x >= 0 && x < width_);
        assert(y >= 0 && y < height_);
        return data_[y * width_ + x];
    }
    bool readWithWrap(int x, int y) const {
        if (x < 0)
            x += width_;
        else if (x >= width_)
            x -= width_;
        if (y < 0)
            y += height_;
        else if (y >= height_)
            y -= height_;
        assert(x >= 0 && x < width_);
        assert(y >= 0 && y < height_);
        return data_[y * width_ + x];
    }
    void write(int x, int y, bool value) {
        assert(x >= 0 && x < width_);
        assert(y >= 0 && y < height_);
        data_[y * width_ + x] = value;
    }
};

unsigned countNeighbours(const Matrix & m, int x, int y) {
    unsigned count = 0;
    for (int row = -1; row <= 1; ++row) {
        for (int col = -1; col <= 1; ++col) {
            if ((col != 0 || row != 0) && m.readWithWrap(x + col, y + row))
                ++count;
        }
    }
    return count;
}

void updateState(const Matrix & oldState, Matrix & newState) {
    for (int y = 0; y < oldState.height(); ++y) {
        for (int x = 0; x < oldState.width(); ++x) {
            unsigned count = countNeighbours(oldState, x, y);
            newState.write(x, y,
                           count == 3 || count == 2 && oldState.read(x, y));
        }
    }
}

void clearScreen() {
    printf("\x1b[2J\x1b[0;0H");
    fflush(stdout);
}

void printState(Matrix & state) {
    if (BENCHMARK)
        return;
    clearScreen();
    int col = 0;
    for (auto b : state) {
        putchar(b ? '*' : '.');
        if (++col == state.width()) {
            col = 0;
            putchar('\n');
        }
    }
}

void delay(unsigned ms) {
    if (BENCHMARK)
        return;
    std::this_thread::sleep_for(std::chrono::milliseconds(ms));
}

std::unique_ptr<Matrix> runSimulation(std::unique_ptr<Matrix> oldState,
                                      size_t steps) {
    auto newState =
        std::make_unique<Matrix>(oldState->width(), oldState->height());
    for (size_t step = 0; step != steps; ++step) {
        updateState(*oldState, *newState);
        printState(*newState);
        delay(50);
        std::swap(oldState, newState);
    }
    return oldState;
}

void stateFromStr(Matrix & state, const char * str) {
    int row = 0;
    int col = 0;
    for (; *str; ++str) {
        if (*str == '\n') {
            col = 0;
            if (++row >= state.height())
                break;
            continue;
        }
        if (col < state.width())
            state.write(col, row, *str == '*');
        ++col;
    }
}

int main() {
    auto state = std::make_unique<Matrix>(WIDTH, HEIGHT);
    stateFromStr(*state, INIT_STATE);
    printState(*state);
    runSimulation(std::move(state), ITERATIONS);
}
