#include <iostream>
#include <map>
#include <fstream>
#include <chrono>


int main() 
{ 
    const int TARGET_MAP_SIZE = 10'000'000;
    std::ofstream output("./results/cpp_map.csv");

    output << "Количество элементов,Время выполнения (мкс),Объем памяти (байт)" << std::endl;

    for (int i = 1; i <= TARGET_MAP_SIZE; i*=10) 
    {
        std::map<int, int> m;

        auto start = std::chrono::steady_clock::now();
        for (int j = 0; j < i; ++j)
        {
            m[j] = j;
        }
        auto end = std::chrono::steady_clock::now();

        auto time = std::chrono::duration_cast<std::chrono::microseconds>(end - start).count();
        output << i << ',' << float(time) << ',' << m.size() * sizeof(int) * 2 << std::endl;
    }

    output.close();

    return 0;
}