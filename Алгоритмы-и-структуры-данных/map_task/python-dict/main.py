import time
import sys
import csv
import os

BASE_DIR = os.getcwd()
TARGET_DICT_SIZE = 100_000  # Количество элементов
OUTPUT_FILEPATH = os.path.join(BASE_DIR, 'python-dict/performance_results.csv')

def measure_performance(n):
    myDict = {}
    times = []
    memoryUsages = []

    start = time.time()

    for i in range(1, n+1):
        myDict[i] = i

        times.append((time.time() - start) * 1000)  # Время выполнения в милисекундах

        # Вычисление требуемого объема памяти
        memoryUsage = sys.getsizeof(myDict)
        memoryUsages.append(memoryUsage)

    return times, memoryUsages

durations, memoryUsages = measure_performance(TARGET_DICT_SIZE)

# Запись результатов в файл CSV
with open(OUTPUT_FILEPATH, mode='w', newline='', encoding='utf-8') as file:
    writer = csv.writer(file)
    writer.writerow(['Количество элементов', 'Время выполнения (мс)', 'Объем памяти (байт)'])
    for i in range(0, TARGET_DICT_SIZE, 100):
        writer.writerow([i+1, durations[i], memoryUsages[i]])

print(f"Результаты измерений сохранены в файле '{OUTPUT_FILEPATH}'.")
