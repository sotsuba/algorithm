import sys

CHUNK_SIZE = 3636


class RepetitionState:
    def __init__(self) -> None:
        self.max_streak = 0
        self.current_streak = 0
        self.last_char = None

    def consume(self, chunk: bytes) -> None:
        curr = self.current_streak
        last = self.last_char
        m_streak = self.max_streak

        for b in chunk:
            if b == last:
                curr += 1
            else:
                curr = 1
                last = b

            if curr > m_streak:
                m_streak = curr

        self.current_streak = curr
        self.max_streak = m_streak
        self.last_char = last


if __name__ == "__main__":
    state = RepetitionState()

    while True:
        chunk = sys.stdin.buffer.read(CHUNK_SIZE)
        if not chunk:
            break
        state.consume(chunk)

    print(state.max_streak)
