def call():
    return "CALL"


def fold():
    return "FOLD"


def raise_by(number: int):
    if number < 0:
        raise ValueError("Raise amount must be a valid unsigned int")
    return f"RAISE {number}"