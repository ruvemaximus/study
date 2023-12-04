import pytest

from special_hash_map import SpecialHashMap


@pytest.fixture()
def hash_map() -> SpecialHashMap:
    return SpecialHashMap()


@pytest.fixture()
def base_hash_map(hash_map) -> SpecialHashMap:
    hash_map['2'] = 2
    hash_map['1'] = 1
    hash_map['3'] = 3
    return hash_map


def test_gte_filter(base_hash_map):
    assert base_hash_map.ploc['>=2'] == {'2': 2, '3': 3}


def test_gt_filter(base_hash_map):
    assert base_hash_map.ploc['>2'] == {'3': 3}


def test_lt_filter(base_hash_map):
    assert base_hash_map.ploc['<3'] == {'1': 1, '2': 2}


def test_lte_filter(base_hash_map):
    assert base_hash_map.ploc['<=2'] == {'1': 1, '2': 2}


def test_neq_filter(base_hash_map):
    assert base_hash_map.ploc['<>2'] == {'1': 1, '3': 3}


def test_str(base_hash_map):
    assert str(base_hash_map) == """{'1': 1, '2': 2, '3': 3}"""


def test_iloc(hash_map: SpecialHashMap):
    hash_map["value1"] = 1
    hash_map["value2"] = 2
    hash_map["value3"] = 3
    hash_map["1"] = 10
    hash_map["2"] = 20
    hash_map["3"] = 30
    hash_map["1, 5"] = 100
    hash_map["5, 5"] = 200
    hash_map["10, 5"] = 300

    assert hash_map.iloc[0] == 10
    assert hash_map.iloc[2] == 300
    assert hash_map.iloc[5] == 200
    assert hash_map.iloc[8] == 3


def test_ploc(hash_map: SpecialHashMap):
    hash_map["value1"] = 1
    hash_map["value2"] = 2
    hash_map["value3"] = 3
    hash_map["1"] = 10
    hash_map["2"] = 20
    hash_map["3"] = 30
    hash_map["(1, 5)"] = 100
    hash_map["(5, 5)"] = 200
    hash_map["(10, 5)"] = 300
    hash_map["(1, 5, 3)"] = 400
    hash_map["(5, 5, 4)"] = 500
    hash_map["(10, 5, 5)"] = 600

    assert hash_map.ploc[">=1"] == {'1': 10, '2': 20, '3': 30}
    assert hash_map.ploc["<3"] == {'1': 10, '2': 20}

    assert hash_map.ploc[">0, >0"] == {'(1, 5)': 100, '(5, 5)': 200, '(10, 5)': 300}
    assert hash_map.ploc[">=10, >0"] == {'(10, 5)': 300}

    assert hash_map.ploc["<5, >=5, >=3"] == {'(1, 5, 3)': 400}
