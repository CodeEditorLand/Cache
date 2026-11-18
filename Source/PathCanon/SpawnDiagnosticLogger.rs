use std::time::Duration;

use crate::PathCanon::Stats;

/// Spawns a background tokio task that logs path-canon cache stats
/// every 30 seconds under the `path-canon` log target.
///
/// The task runs indefinitely until the tokio runtime shuts down.
/// Call from the embedder's runtime setup when diagnostics are
/// desired.
///
/// Calling this outside an active Tokio runtime logs a warning on the
/// `path-canon` target and does nothing - diagnostics are optional, so a
/// missing reactor degrades to a documented no-op rather than panicking.
///
/// # Examples
///
/// ```rust,no_run
/// use land_cache::PathCanon::SpawnDiagnosticLogger;
///
/// SpawnDiagnosticLogger::Fn();
/// ```
pub fn Fn() {
	// `tokio::spawn` panics without an active runtime. Check first so a
	// non-Tokio caller warns and degrades instead of bringing the process down.
	match tokio::runtime::Handle::try_current() {
		Ok(Runtime) => {
			// Runtime present: preserve the original spawn behaviour. The
			// `JoinHandle` is discarded here; returning one would change the
			// public `Fn()` signature (tracked as a follow-up).
			let _ = Runtime.spawn(async {
				let mut Interval = tokio::time::interval(Duration::from_secs(30));

				Interval.tick().await;

				loop {
					Interval.tick().await;

					let Snapshot = Stats::Fn();

					log::debug!(
						target:"path-canon",

						"entries={} weighted={}",

						Snapshot.Entries,

						Snapshot.WeightedSize
					);
				}
			});
		},
		Err(Error) => {
			// No reactor to drive the logger: warn and take the no-op path
			// rather than panicking. Diagnostics are optional by design.
			log::warn!(
				target:"path-canon",

				"no Tokio runtime available for path-canon diagnostic logger ({}); skipping periodic telemetry",
				Error
			);
		},
	}
}
