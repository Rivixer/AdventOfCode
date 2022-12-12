class PiorityQueue:
    def __init__(self):
        self.__queue: list[tuple[int, tuple[int, int]]] = list()

    def __bool__(self):
        return bool(len(self.__queue))

    def add(self, priority: int, item: tuple[int, int]):
        if not len(self.__queue):
            return self.__queue.append((priority, item))

        if priority < self.__queue[0][0]:
            return self.__queue.insert(0, (priority, item))

        for i in range(len(self.__queue)-1):
            if self.__queue[i][0] <= priority <= self.__queue[i+1][0]:
                return self.__queue.insert(i+1, (priority, item))

        self.__queue.append((priority, item))

    def get(self):
        return self.__queue.pop(0)

    def __len__(self):
        return len(self.__queue)

    def __iter__(self):
        yield from self.__queue
