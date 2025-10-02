# https://www.codewars.com/kata/588a00ad70720f2cd9000005/train/python


class Router(object):
	def __init__(self):
		self._routes = {}

	def bind(self, url, method, action ):
		self._routes[f"{method} {url}"] = action

	def runRequest(self, url, method):
		try:
			return self._routes[f"{method} {url}"].__call__()
		except KeyError:
			return "Error 404: Not Found"

if __name__ == "__main__":
	router = Router()
	router.bind("/hello", "GET", lambda: "hello world")

	request = router.runRequest("/hello", "GET")
	print(request)

	request = router.runRequest("/", "GET")
	print(request)
