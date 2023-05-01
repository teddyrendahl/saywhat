import saywhat


def test_edit_distance():
    assert saywhat.edit_distance("kitten", "sitting", 1, False) == 3


def test_edit_distance_with_transpositions():
    assert saywhat.edit_distance("ab", "ba", 1, True) == 1
