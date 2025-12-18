-- this script is used to verify background service worker can cleanup old sessions.
INSERT INTO sessions (id, token, user_id, expires_at)
VALUES
    (X'019b33907bef7aa19f24d5a7406a8adb', 'empty', X'019b3387d4a072b08c4bbf86bae7ffe2', CURRENT_TIMESTAMP),
    (X'019b3390850f74b1b772d7d02c6d08c7', 'empty', X'019b3387d4a072b08c4bbf86bae7ffe2', CURRENT_TIMESTAMP),
    (X'019b339087aa7213a352f63305b83735', 'empty', X'019b3387d4a072b08c4bbf86bae7ffe2', CURRENT_TIMESTAMP),
    (X'019b339089de74b29d071a434ebfc829', 'empty', X'019b3387d4a072b08c4bbf86bae7ffe2', CURRENT_TIMESTAMP),
    (X'019b33908b967b5288e3387c8cf5f67c', 'empty', X'019b3387d4a072b08c4bbf86bae7ffe2', CURRENT_TIMESTAMP),
    (X'019b33913c2a7fe189f371455e08a911', 'empty', X'019b3387d4a072b08c4bbf86bae7ffe2', CURRENT_TIMESTAMP);