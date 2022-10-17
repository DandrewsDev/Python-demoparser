### Examples

All examples are multiprocessed. This will mostly just mean these 2 lines of code:
```python
with mp.Pool(processes=8) as pool:
        results = pool.map(parse_function, files)
```
Or with a progress bar:
```python
with mp.Pool(processes=8) as pool:
        results = list(tqdm.tqdm(pool.imap_unordered(parse_function, files), total=len(files)))
```
This spawns a group of "workers" that process you input list in parallel. Each worker takes one item from your list and passes it to your function and runs the function.

### Note about safety
DO NOT MODIFY VARIABLES OUTSIDE YOUR PARALLEL FUNCTION. This should be the only way you can go wrong. This mostly just means do not use global variables or "self".

#### Example of bad:
```python

total_kills = 0

def parse(file):
        parser = DemoParser(file)
        events = parser.parse_events("player_death")
        # BAD. total kills is a variable outside of our func
        total_kills += len(events)

with mp.Pool(processes=8) as pool:
        results = pool.map(parse_function, files)
print(sum(total_kills))
```
#### Solution:
```python

def parse(file):
        parser = DemoParser(file)
        events = parser.parse_events("player_death")
        return len(events)

with mp.Pool(processes=8) as pool:
        results = pool.map(parse_function, files)
print(sum(results))
```




Since demo parsing is "embarrassingly parallel", meaning we dont need to communicate between demos, we can avoid the main dangers of parallel computing. The majoriy of horror stories about parallel computing comes from tasks that are NOT easily parallelizable.