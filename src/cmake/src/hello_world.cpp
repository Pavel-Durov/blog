#include <iostream>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

int main()
{
    std::cout << nlohmann::json::parse("{ \"msg\": \"Hello, World!\" }") << std::endl;
    return 0;
}
