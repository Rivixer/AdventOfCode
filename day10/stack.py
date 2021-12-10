class Stack:
    def __init__(self):
        self._stack = []

    def add_item(self, item):
        self._stack.append(item)

    def remove_last_item(self):
        return self._stack.pop(-1)