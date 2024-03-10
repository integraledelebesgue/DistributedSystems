from pydantic import BaseModel, Field
from enum import Enum
from itertools import count
from typing import Optional, Any, Literal


class Status:
    def dump(self) -> dict[Literal['status', 'error'], str]:
        key = 'status'\
            if self.name == 'Ok'\
            else 'error'

        return {key: self.value}


class VoteStatus(Status, Enum):
    Ok = 'Ok'
    NotFound = 'Vote not found'

    
class PollStatus(Status, Enum):
    Ok = 'Ok'
    NotFound = 'Poll not found'


class Poll(BaseModel):
    question: str
    options: list[str]
    votes: dict[int, int] = Field(default_factory=dict)

    def _delete_illegal_votes(self) -> None:
        n_options = len(self.options)
        
        self.votes = dict(
            (key, value) 
            for (key, value) in self.votes.items()
            if 0 <= value and value < n_options
        )

    def model_post_init(self, __context: Any) -> None:
        self._delete_illegal_votes()

    def _verify_vote(self, option: int) -> bool:
        return 0 <= option and option < len(self.options)
    
    def _next_id(self) -> int:
        return max(self.votes.keys(), default=-1) + 1
    
    def get_vote(self, id: int) -> Optional[int]:
        return self.votes.get(id)

    def add_vote(self, option: int) -> VoteStatus:
        if not self._verify_vote(option):
            return VoteStatus.NotFound
        
        self.votes[self._next_id()] = option

        return VoteStatus.Ok
    
    def update_vote(self, id: int, value: int) -> VoteStatus:
        if not self._verify_vote(value):
            return VoteStatus.NotFound
        
        self.votes[id] = value
        
        return VoteStatus.Ok

    def delete_vote(self, id: int) -> VoteStatus:
        if id not in self.votes:
            return VoteStatus.NotFound
        
        self.votes.pop(id)

        return VoteStatus.Ok

    def summarize_and_dump(self) -> dict[str, int]:
        options = range(len(self.options))
        votes = self.votes.items()
        
        return {
            str(option): len([
                entry 
                for entry in votes 
                if entry[1] == option
            ])
            for option in options
        }


class PollStorage:
    polls: dict[int, Poll]
    _ids: count

    def __init__(self) -> None:
        self.polls = dict()
        self._ids = count()

    def dump(self) -> dict[str, dict[str, Any]]:
        return {
            str(id): self.polls[id].model_dump()
            for id in self.polls.keys()
        }

    def get(self, id: int) -> Optional[Poll]:
        return self.polls.get(id)

    def add(self, poll: Poll) -> PollStatus:
        id = next(self._ids)
        self.polls[id] = poll

        return PollStatus.Ok
    
    def update(self, id: int, new: Poll) -> PollStatus:
        if id not in self.polls:
            return PollStatus.NotFound
        
        self.polls[id] = new

        return PollStatus.Ok

    def delete(self, id: int) -> PollStatus:
        if id not in self.polls:
            return PollStatus.NotFound
        
        self.polls.pop(id)

        return PollStatus.Ok

