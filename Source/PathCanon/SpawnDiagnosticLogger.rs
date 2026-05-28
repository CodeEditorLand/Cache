//! Spawn a tokio task that logs cache stats every 30s under the
//! `path-canon` target. Optional - call from the embedder's runtime
//! setup when the user has the `path-canon` tag enabled.

use std::time::Duration;

use crate::PathCanon::Stats;

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
