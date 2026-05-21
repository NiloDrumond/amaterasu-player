<script lang="ts">
	import AppSidebar from '$lib/components/app-sidebar.svelte';
	import PlayerBar from '$lib/components/player/player-bar.svelte';
	import CommandPalette from '$lib/components/search/command-palette.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import { setPlayer } from '$lib/player/player.svelte';
	import {
		hydratePlayerFromStorage,
		setupPlayerPersistence,
	} from '$lib/player/player-persistence.svelte';
	import { usePlayerShortcuts } from '$lib/shortcuts/player-shortcuts.svelte';
	import { getVolume, initUserPreferences, setVolume } from '$lib/state/user-preferences.svelte';

	let { data, children } = $props();
	const player = setPlayer();
	usePlayerShortcuts(player);
	// svelte-ignore state_referenced_locally
	initUserPreferences(data.user.preferences);
	const initialVolume = getVolume();
	if (initialVolume !== null) {
		player.volume = initialVolume;
		player.previousVolume = initialVolume > 0 ? initialVolume : 1;
	}
	hydratePlayerFromStorage(player);
	setupPlayerPersistence(player);
	$effect(() => {
		const v = player.volume;
		if (v !== getVolume()) setVolume(v);
	});
</script>

<Sidebar.Provider>
	<AppSidebar
		class={player.currentTrack ? 'pb-20' : undefined}
		user={data.user}
		recentPlaylists={data.recentPlaylists}
	/>
	<Sidebar.Inset class={player.currentTrack ? 'pb-20' : undefined}>
		{@render children()}
	</Sidebar.Inset>
</Sidebar.Provider>
<PlayerBar />
<CommandPalette />
