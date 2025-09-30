# https://www.codewars.com/kata/515bb423de843ea99400000a/train/python


import math

class PaginationHelper:
	def __init__(self, collection, items_per_page):
		self.collection = collection
		self.items_per_page = items_per_page
		self.pages = [self.collection[i:i+self.items_per_page] for i in range(0, len(self.collection), self.items_per_page)]

	def item_count(self):
		return len(self.collection)

	def page_count(self):
		return math.ceil(len(self.collection) / self.items_per_page)

	def page_item_count(self, page_index):
		if page_index < 0:
			return - 1
		try:
			return len(self.pages[page_index])
		except IndexError:
			return -1

	def page_index(self, item_index):
		page_index_ = 0
		all_index = 0

		for pages in self.pages:
			for _ in pages:
				if item_index == all_index:
					return page_index_
				all_index += 1
			page_index_ += 1

		return - 1


if __name__ == "__main__":
	helper = PaginationHelper(['a','b','c','d','e','f'], 4)
	helper.page_count()
	helper.item_count()
	helper.page_item_count(0)
	helper.page_item_count(1)
	helper.page_item_count(2)

	helper.page_index(5)
	helper.page_index(2)
	helper.page_index(20)
	helper.page_index(-10)

