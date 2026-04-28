<script lang="ts">
	type AlbumCoverCellProps = {
		id: string;
		title: string;
		coverUrl: string | null;
	};
	let { id, title, coverUrl }: AlbumCoverCellProps = $props();
</script>

<a href={`/albums/${id}`} class="flex flex-row items-center gap-2">
	<img class="size-8" src={coverUrl ?? undefined} alt={title} />
	<span class="overflow-hidden text-ellipsis">
		{title}
	</span>
</a>
