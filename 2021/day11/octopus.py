class Octopus:
    def __init__(self, energy: int):
        self._energy = energy
        self._flashed = False
        self._flash_count = 0

    def __str__(self):
        return f'{self._energy=}, {self._flashed=}'

    def __repr__(self):
        return f'{self._energy}/{"T" if self._flashed else "F"}'

    def next_step(self):
        self._flashed = False
        self._energy += 1
    
    def can_flash(self):
        return self._energy >= 10

    def flash(self):
        self._flashed = True
        self._energy = 0
        self._flash_count += 1

    def flashed_in_this_round(self):
        return self._flashed

    def add_energy(self):
        self._energy += 1
        
