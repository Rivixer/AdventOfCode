from typing import List, Tuple


class PiorityQueue:
    def __init__(self):
        self.queue: List[Tuple[Tuple[int, int], int]] = list()

    def __bool__(self):
        return bool(len(self.queue))

    def add(self, item: Tuple[int, int], priority: int):
        if not len(self.queue):
            return self.queue.append((priority, item))

        if priority < self.queue[0][0]:
            return self.queue.insert(0, (priority, item))

        for i in range(len(self.queue)-1):
            if self.queue[i][0] <= priority <= self.queue[i+1][0]:
                return self.queue.insert(i+1, (priority, item))
                
        self.queue.append((priority, item))
    
    def get(self):
        return self.queue.pop(0)[1]

    def __len__(self):
        return len(self.queue)
