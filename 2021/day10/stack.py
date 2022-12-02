class Stack:
    def __init__(self):
        self._stack = []

    def add_item(self, item):
        self._stack.append(item)

    def remove_last_item(self):
        return self._stack.pop(-1)

    def get_all_items(self):
        return self._stack

    def __str__(self):
        return ''.join(self._stack)