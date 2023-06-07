import time
import sys
import csv
import os
import datetime

BASE_DIR = os.getcwd()
TARGET_DICT_SIZE = 100_000  # Количество элементов
OUTPUT_FILEPATH = os.path.join(BASE_DIR, 'results/python_dict.csv')

def measure_performance(n):
    times = []
    memoryUsages = []
    
    i = 1
    while i <= TARGET_DICT_SIZE:
        myDict = {}
        start = datetime.datetime.now().microsecond

        for j in range(i+1):
            myDict[j] = j

        end = datetime.datetime.now().microsecond
        duration = end - start
        times.append(duration)

        # Вычисление требуемого объема памяти
        memoryUsage = sys.getsizeof(myDict)
        memoryUsages.append(memoryUsage)
        i *= 10

    return times, memoryUsages

durations, memoryUsages = measure_performance(TARGET_DICT_SIZE)

# Запись результатов в файл CSV
with open(OUTPUT_FILEPATH, mode='w', newline='', encoding='utf-8') as file:
    writer = csv.writer(file)
    writer.writerow(['Количество элементов', 'Время выполнения (мкс)', 'Объем памяти (байт)'])
    for i in range(len(durations)):
        writer.writerow([10**(i+1), durations[i], memoryUsages[i]])
