# https://www.codewars.com/kata/687dd1e199ab3955339b8071/train/python


def can_play(hand, face_up):
	fc, fn = face_up.split(" ")

	for card in hand:
		cc, cn = card.split(" ")
		if (cc == fc) or (cn == fn):
			return True

	return False
