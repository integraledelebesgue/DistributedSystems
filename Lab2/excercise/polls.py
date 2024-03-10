from fastapi import FastAPI
from typing import Any

from models import Poll, VoteStatus, PollStatus, PollStorage


app = FastAPI()

polls = PollStorage()


@app.get('/polls')
async def get_polls() -> dict[int, dict[str, Any]]:
    return polls.dump()

@app.get('/polls/{id}')
async def get_poll(id: int) -> dict[str, Any]:
    if poll := polls.get(id):
        return poll.model_dump()
    
    return PollStatus.NotFound.dump()

@app.get('/polls/{id}/votes')
async def get_votes(id: int) -> dict[str, Any]:
    if poll := polls.get(id):
        return {'votes': poll.votes}
    
    return PollStatus.NotFound.dump()

@app.get('/polls/{id}/votes/stats')
async def get_vote_stats(id: int) -> dict[str, Any]:
    if poll := polls.get(id):
        return poll.summarize_and_dump()
    
    return PollStatus.NotFound.dump()

@app.get('/polls/{poll_id}/votes/{vote_id}')
async def get_vote(poll_id: int, vote_id: int) -> dict[str, Any]:
    poll = polls.get(poll_id)

    if poll is None:
        return PollStatus.NotFound.dump()
    
    if id := poll.get_vote(vote_id):
        return {'vote': id}
        
    return VoteStatus.NotFound.dump()


@app.post('/polls')
async def create_poll(poll: Poll) -> dict[str, str]:
    return polls\
        .add(poll)\
        .dump()

@app.post('/polls/{id}/votes')
async def vote(id: int, option: int) -> dict[str, str]:
    poll = polls.get(id)

    if poll is None:
        return PollStatus.NotFound.dump()
    
    return poll\
        .add_vote(option)\
        .dump()


@app.put('/polls/{id}')
async def update_poll(id: int, new: Poll) -> dict[str, str]:
    return polls\
        .update(id, new)\
        .dump()
    
@app.put('/polls/{poll_id}/vote/{vote_id}')
async def update_vote(poll_id: int, vote_id: int, value: int):
    poll = polls.get(poll_id)

    if poll is None:
        return PollStatus.NotFound.dump()
    
    return poll\
        .update_vote(vote_id, value)\
        .dump()


@app.delete('/polls/{id}')
async def delete_poll(id: int) -> dict[str, str]:
    return polls\
        .delete(id)\
        .dump()

@app.delete('/polls/{poll_id}/votes/{vote_id}')
async def delete_vote(poll_id: int, vote_id: int) -> dict[str, str]:
    poll = polls.get(poll_id)

    if poll is None:
        return PollStatus.NotFound.dump()
    
    return poll\
        .delete_vote(vote_id)\
        .dump()

