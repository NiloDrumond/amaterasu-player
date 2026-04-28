<script lang="ts">
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';
	import { formatQuality } from '$lib/utils/audio';
	import Badge from '../ui/badge/badge.svelte';

	type TrackQualityProps = {
		track: TrackResponse;
	};
	let { track }: TrackQualityProps = $props();
</script>

<Badge variant="outline">
	{formatQuality(track.format, track.codec, track.bitrate)}
</Badge>
