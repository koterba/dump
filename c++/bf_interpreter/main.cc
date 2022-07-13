#include <iostream>
#include <sstream>
#include <fstream>
#include <vector>

enum Op {
    INCPTR, // '>' = move pointer to the right
    DECPTR, // '<' = move pointer to the left
    INCMEM, // '+' = increase current mem value at ptr location
    DECMEM, // '-' = decrease current mem value at ptr location
    OUTPUT, // '.' = output char at current ptr location
    TAKEIN, // ',' = take input
    LOOPST, // '[' = loop start
    LOOPND  // ']' = loop end
};

std::vector<Op> parse_input(std::string input) {
    std::vector<Op> tokens;

    for (auto op : input) {
        switch (op) {
            case '>': tokens.push_back(Op::INCPTR); break;
            case '<': tokens.push_back(Op::DECPTR); break;
            case '+': tokens.push_back(Op::INCMEM); break;
            case '-': tokens.push_back(Op::DECMEM); break;
            case '.': tokens.push_back(Op::OUTPUT); break;
            case ',': tokens.push_back(Op::TAKEIN); break;
            case '[': tokens.push_back(Op::LOOPST); break;
            case ']': tokens.push_back(Op::LOOPND); break;
            default: break;
        }
    }

    return tokens;
}

int run(std::vector<Op> instructions) {
    std::vector<int> loop_start;
    int instructions_ptr = 0;
    char output_buff;
    int input_buff;
    int memory[30000];
    int ptr = 0;

    while (true) {
        switch (instructions[instructions_ptr]) {
            case Op::INCPTR:
                ptr++;
                break;

            case Op::DECPTR:
                ptr--;
                break;

            case Op::INCMEM:
                memory[ptr]++;
                break;

            case Op::DECMEM:
                memory[ptr]--;
                break;

            case Op::OUTPUT:
                output_buff = memory[ptr];
                std::cout << output_buff << " " << std::flush;
                break;

            case Op::TAKEIN:
                std::cin >> input_buff;
                memory[ptr] = input_buff;
                break;

            case Op::LOOPST:
                loop_start.push_back(instructions_ptr);
                break;

            case Op::LOOPND:
                if (memory[ptr] <= 0) {
                    loop_start.pop_back();
                } else {
                    instructions_ptr = loop_start.back();
                }
                break;
        }

        if (instructions_ptr > instructions.size()) {
            break;
        }
        //std::cout << memory[ptr] << std::endl;

        instructions_ptr++;
    }

    return 0;
}

std::string parseFile(std::string filepath) {
    auto string_buff = std::ostringstream{};
    std::ifstream file(filepath);
    if (!file.is_open()) {
        std::cerr << "Could not open the file - '"
                << filepath << "'" << std::endl;
        exit(EXIT_FAILURE);
    }
    string_buff << file.rdbuf();
    return string_buff.str();
}

int main(int argc, char** argv) {
    if (argc < 2) {
        std::cout << "ERROR: filename not provided as argument" << std::endl;
        return 1;
    }

    std::string input = parseFile(argv[1]);
    std::vector<Op> instructions = parse_input(input);
    run(instructions);
}