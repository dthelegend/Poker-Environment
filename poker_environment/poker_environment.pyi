class PyPokerEnvironment:
    table_cards: list[str]
    current_player: PyPokerPlayer
    player_states: list[PyPokerPlayerVisible]
    game_history: list[list[str]]
    pot: int
    minimum_bet: int
    expected_bet: int

class PyPokerPlayer:
    hand: list[str]
    remaining_balance: int
    committed_balance: int

class PyPokerPlayerVisible:
    remaining_balance: int
    committed_balance: int