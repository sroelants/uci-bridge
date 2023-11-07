### Usage
Run the bridge, passing the binary of a chess engine as the argument (either a binary in your
`$PATH`, or the path to the executable)

```sh
$ uci-bridge stockfish
```

This starts a server listening on `localhost:3000` that relays any uci commands
you give it and pipes the output back to you

```sh
$ curl -X POST --data "uci" http://127.0.0.1:3000
Stockfish 16 by the Stockfish developers (see AUTHORS file)
id name Stockfish 16
id author the Stockfish developers (see AUTHORS file)

option name Debug Log File type string default
option name Threads type spin default 1 min 1 max 1024
option name Hash type spin default 16 min 1 max 33554432
option name Clear Hash type button
option name Ponder type check default false
option name MultiPV type spin default 1 min 1 max 500
option name Skill Level type spin default 20 min 0 max 20
option name Move Overhead type spin default 10 min 0 max 5000
option name Slow Mover type spin default 100 min 10 max 1000
option name nodestime type spin default 0 min 0 max 10000
option name UCI_Chess960 type check default false
option name UCI_AnalyseMode type check default false
option name UCI_LimitStrength type check default false
option name UCI_Elo type spin default 1320 min 1320 max 3190
option name UCI_ShowWDL type check default false
option name SyzygyPath type string default <empty>
option name SyzygyProbeDepth type spin default 1 min 1 max 100
option name Syzygy50MoveRule type check default true
option name SyzygyProbeLimit type spin default 7 min 0 max 7
option name Use NNUE type check default true
option name EvalFile type string default nn-5af11540bbfe.nnue
uciok⏎
```
