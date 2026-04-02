Real issues worth fixing:

  1. Potential deadlock
  The worker holds the state lock and calls advance(), which then tries to lock cache. Meanwhile the main thread could be holding cache and waiting for state.
  Two different mutexes acquired in different orders = classic deadlock.

  2. long_break_interval = 0 panics
  self.sessions_count % self.config.long_break_interval() // modulo by zero
  The default is 4 so it won't happen normally, but bad config crashes the app.

  3. Tick interval toggle latency
  When you press m to toggle millis, the worker could be mid-sleep for 1000ms, so the display won't update at 10ms rate for up to 1 second after the toggle.

  4. Duration precision loss
  Sessions store duration_millis / 1000 — any sub-second remainder is silently dropped. Minor since phase durations are whole minutes, but worth noting.
