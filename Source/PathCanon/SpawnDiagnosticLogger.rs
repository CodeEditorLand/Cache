use std::time::Duration;

use crate::PathCanon::Stats;

/// Spawns a background tokio task that logs path-canon cache stats
/// every 30 seconds under the `path-canon` log target.
///
/// The task runs indefinitely until the tokio runtime shuts down.
/// Call from the embedder's runtime setup when diagnostics are
/// desired.
///
/// # Examples
///
/// ```rust,no_run
/// use land_cache::PathCanon::SpawnDiagnosticLogger;
///
/// SpawnDiagnosticLogger::Fn();
/// ```
pub fn Fn() {
	tokio::spawn(async {
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
}
