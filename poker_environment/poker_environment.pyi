class PyPokerEnvironment:
    table_cards: list[str]
    current_player: PyPokerDealtPlayer
    player_states: list[PyPokerPlayerVisible]
    game_history: list[list[str]]
    pot: int
    minimum_bet: int
    expected_bet: int

class PyPokerDealtPlayer:
    player_id: str
    remaining_balance: int
    committed_balance: int
    hand: list[str]

class PyPokerPlayerVisible:
    remaining_balance: int
    committed_balance: int