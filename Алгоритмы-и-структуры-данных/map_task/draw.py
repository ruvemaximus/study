import matplotlib.pyplot as plt
import csv


class MeasureResult:
    def __init__(
        self,
        elems: list,
        times: list,
        memories: list
    ):
        self.elems = list(map(int, elems[1:]))
        self.times = list(map(float, times[1:]))
        self.memories = list(map(float, memories[1:]))



def read_from(filename: str): 
    with open(filename, 'r', encoding='utf-8') as file:
        csv_file = csv.reader(file)

        elems = []
        times = []
        memories = []

        for line in csv_file: 
            elem, time, memory = line

            elems.append(elem)
            times.append(time)
            memories.append(memory)

        return MeasureResult(elems, times, memories)
    

CPP = read_from('./results/cpp_map.csv')
PYTHON = read_from('./results/python_dict.csv')

plt.subplot(2, 2, 1)
plt.plot(CPP.elems, CPP.times)
plt.xlabel('Кол-во элементов')
plt.ylabel('Время (мкс)')
plt.title('C++ Время от количества')

plt.subplot(2, 2, 2)
plt.plot(CPP.elems, CPP.memories)
plt.xlabel('Кол-во элементов')
plt.ylabel('Память (байты)')
plt.title('C++ Память от количества')

plt.subplot(2, 2, 3)
plt.plot(PYTHON.elems, PYTHON.times)
plt.xlabel('Кол-во элементов')
plt.ylabel('Время (мкс)')
plt.title('Python Время от количества')

plt.subplot(2, 2, 4)
plt.plot(PYTHON.elems, PYTHON.memories)
plt.xlabel('Кол-во элементов')
plt.ylabel('Память (байты)')
plt.title('Python Память от количества')


plt.show()