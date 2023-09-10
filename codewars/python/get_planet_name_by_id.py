# https://www.codewars.com/kata/515e188a311df01cba000003/train/python


def get_planet_name(id):
    return [
        "Mercury",
        "Venus",
        "Earth",
        "Mars",
        "Jupiter",
        "Saturn",
        "Uranus",
        "Neptune",
    ][id - 1]


if __name__ == "__main__":
    print(get_planet_name(2))
