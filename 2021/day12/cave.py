class Cave:
    def __init__(self, name: str):
        self.name = name
        self.is_small = name.islower() and name not in ('start', 'end')
        self.connections = set()

    def add_connection(self, connection):
        self.connections.add(connection)

    def __repr__(self):
        return self.name

    def __str__(self):
        return self.name
