<script lang="ts">
	import AppSidebar from '$lib/components/app-sidebar.svelte';
	import PlayerBar from '$lib/components/player/player-bar.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import { setPlayer } from '$lib/player/player.svelte';
	import { initUserPreferences } from '$lib/state/user-preferences.svelte';

	let { data, children } = $props();
	const player = setPlayer();
	// svelte-ignore state_referenced_locally
	initUserPreferences(data.user.preferences);
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
