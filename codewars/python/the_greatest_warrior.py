#!/usr/bin/env python3


# https://www.codewars.com/kata/5941c545f5c394fef900000c/train/python


class Warrior:
    ranks = [ "Pushover", "Novice", "Fighter", "Warrior", "Veteran", "Sage", "Elite", "Conqueror", "Champion", "Master", "Greatest" ]

    def __init__(self):
        self.level = 1
        self.experience = 100
        self.rank = self.ranks[0]
        self.achievements = []

    def training(self, training):
        desc, exp, min_level = training

        if self.level < min_level:
            return "Not strong enough"

        if self.level >= min_level:
            self.experience += exp
            if self.experience > 10000:
                self.experience = 10000
            if self.level > 100:
                self.level = 100
            self.level = self.experience // 100
            self.rank = self.ranks[10 if (self.level // 10) > 10 else self.level // 10]
            self.achievements.append(desc)

        return desc

    def battle(self, battle):
        if battle <= 0 or battle > 100:
            return "Invalid level"

        diff_level = self.level - battle
        diff_rank = (self.level // 10) - (battle // 10)

        result = "Easy fight"

        if diff_level == 0:
            self.experience += 10
            result = "A good fight"
        elif diff_level == 1:
            self.experience += 5
            result = "A good fight"
        elif (diff_level <= -5) and (diff_rank <= -1):
            result = "You've been defeated"
        elif diff_level <= -1 and diff_rank <= 0:
            self.experience += (20 * diff_level * diff_level)
            result = "An intense fight"

        self.level = self.experience // 100
        self.rank = self.ranks[10 if (self.level // 10) > 10 else self.level // 10]

        return result


if __name__ == "__main__":
	bruce_lee = Warrior()

	# 1
	print(f"out: {bruce_lee.level}")

	# 100
	print(f"out: {bruce_lee.experience}")

	# "Pushover"
	print(f"out: {bruce_lee.rank}")

	# []
	print(f"out: {bruce_lee.achievements}")

	# "Defeated Chuck Norris"
	print(f"out: {bruce_lee.training(["Defeated Chuck Norris", 9000, 1])}")

	# 9100
	print(f"out: {bruce_lee.experience}")

	# 91
	print(f"out: {bruce_lee.level}")

	# "Master"
	print(f"out: {bruce_lee.rank}")

	# "A good fight"
	print(f"out: {bruce_lee.battle(90)}")

	# 9105
	print(f"out: {bruce_lee.experience}")

	# ["Defeated Chuck Norris"]
	print(f"out: {bruce_lee.achievements}")
