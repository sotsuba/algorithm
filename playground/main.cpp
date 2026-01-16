#include <iostream>
#include <string> 
#include <string_view>

struct RepetitionState {
    char last_char = '\0';
    size_t current_streak = 0;
    size_t max_streak = 0;

    void consume(std::string_view chunk) {
        for (char c: chunk) {
            if (c == last_char) 
                current_streak++;    
            else {
                current_streak = 1;
                last_char = c;
            }
            max_streak = std::max(max_streak, current_streak);
        }
    }
};


int main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    RepetitionState state;

    char buffer[3636];
    while (std::cin.read(buffer, sizeof(buffer)) || std::cin.gcount() > 0) {
        state.consume(std::string_view(buffer, std::cin.gcount()));
    }

    std::cout << state.max_streak << '\n';
}

// https://cses.fi/problemset/task/1069/
