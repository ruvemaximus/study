from special_hash_map import SpecialHashMap

if __name__ == '__main__':
    m = SpecialHashMap()
    m["value2"] = 2
    m["value3"] = 3
    m["1"] = 10
    m["2"] = 20
    m["3"] = 30
    m["(1, 5)"] = 100
    m["(5, 5)"] = 200
    m["(10, 5)"] = 300
    m["(1, 5, 3)"] = 400
    m["(5, 5, 4)"] = 500
    m["(10, 5, 5)"] = 600

    print(m.ploc[">0, >0"])  # {'(1, 5)': 100, '(5, 5)': 200, '(10, 5)': 300}
    print(m.ploc[">=10, >0"])  # {'(10, 5)': 300}
