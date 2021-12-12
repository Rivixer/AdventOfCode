class Stack:
    def __init__(self, stack: list = list(), visited_small_2_times = False):
        self._stack = stack
        self.visited_small_2_times = visited_small_2_times

    def it_is_last_item(self, item):
        return item == self._stack[-1]

    def add_item(self, item):
        self._stack.append(item)

    def remove_last_item(self):
        return self._stack.pop(-1)

    def get_all_items(self) -> list:
        return self._stack

    def __str__(self):
        return  ' '.join(list(map(lambda i: str(i), self._stack)))

    def __len__(self):
        return len(self._stack)