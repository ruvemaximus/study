import re
from copy import deepcopy
from typing import List


class Filter:
    def __init__(self, data: 'SpecialHashMap'):
        self.data = data

    @staticmethod
    def get_filter_func(__filter: str) -> callable:
        op, value = re.findall(r'([>=<]{1,2})(\d+\.?\d*)', __filter)[0]
        value = float(value)
        match op:
            case '>':
                return lambda x: x > value
            case '<':
                return lambda x: x < value
            case '<>':
                return lambda x: x != value
            case '<=':
                return lambda x: x <= value
            case '>=':
                return lambda x: x >= value

    def __getitem__(self, item: str):
        delimiter = ','
        filters = item.replace(' ', '').split(delimiter)

        # Получаем подходящие ключи под фильтр
        keys: List[str] = [key for key in self.data.base.keys() if len(key.split(delimiter)) == len(filters)]

        result = {}
        for key in keys:
            try:
                values: List[float] = list(map(float, re.sub(r'[() ]', '', key).split(delimiter)))
            except ValueError:
                continue

            if all([self.get_filter_func(f)(v) for f, v in zip(filters, values)]):
                result[key] = self.data.base[key]

        return result


class SpecialHashMap:
    def __init__(self):
        self.base = {}

    def __setitem__(self, key, value) -> None:
        self.base[key] = value
        self.base = dict(sorted(self.base.items()))

    def __str__(self):
        return self.base.__str__()

    @property
    def iloc(self) -> list:
        return list(self.base.values())

    @property
    def ploc(self) -> Filter:
        return Filter(self)
