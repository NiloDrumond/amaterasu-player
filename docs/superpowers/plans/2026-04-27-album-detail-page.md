# Album Detail Page Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the `/albums/[id]` detail page with album hero section, action toolbar (play/shuffle/play-next/play-later/add-to-playlist), and a tracks table.

**Architecture:** Two new backend endpoints (`GET /api/albums/:id` and `GET /api/albums/:id/tracks`) feed a SvelteKit server load that fetches both in parallel. The page renders a hero section (cover + metadata), a toolbar wired to `PlayerState`, and the existing `DataTable` with `tracksColumns`. Two new methods (`playNext`, `playLater`) are added to `PlayerState`.

**Tech Stack:** Axum (Rust), SvelteKit 5, Svelte 5 runes, Tailwind CSS 4, shadcn-svelte, TanStack Table, `@lucide/svelte` icons.

---

## File Map

| Status | Path | What changes |
|--------|------|--------------|
| Modify | `apps/server/src/repositories/track_repository.rs` | Add `find_by_album_id` |
| Modify | `apps/server/src/services/library_service.rs` | Add `get_album_by_id`, `get_tracks_by_album_id` |
| Modify | `apps/server/src/handlers/albums_handlers.rs` | Add `get_album`, `get_album_tracks` handlers |
| Modify | `apps/server/src/routes/album_routes.rs` | Register two new routes |
| Modify | `apps/web/src/lib/player/player.svelte.ts` | Add `playNext`, `playLater` |
| Create | `apps/web/src/routes/(protected)/albums/[id]/+page.server.ts` | Server load |
| Create | `apps/web/src/routes/(protected)/albums/[id]/+page.svelte` | Detail page UI |

---

## Task 1: Add `TrackRepository::find_by_album_id`

**Files:**
- Modify: `apps/server/src/repositories/track_repository.rs`

- [ ] **Step 1: Add the method**

  Open `apps/server/src/repositories/track_repository.rs`. After the `find_all` method, add:

  ```rust
  pub async fn find_by_album_id(
      executor: impl PgExecutor<'_>,
      album_id: Uuid,
  ) -> AppResult<Vec<Track>> {
      let tracks = sqlx::query_as!(
          Track,
          r#"
          SELECT
              *
          FROM
              tracks
          WHERE
              album_id = $1
          ORDER BY
              disc NULLS LAST,
              track_no NULLS LAST
          "#,
          album_id
      )
      .fetch_all(executor)
      .await?;

      Ok(tracks)
  }
  ```

- [ ] **Step 2: Verify it compiles**

  ```bash
  cd apps/server && cargo check 2>&1 | grep -E "^error"
  ```

  Expected: no output (no errors).

- [ ] **Step 3: Commit**

  ```bash
  git add apps/server/src/repositories/track_repository.rs
  git commit -m "feat: add TrackRepository::find_by_album_id"
  ```

---

## Task 2: Add `LibraryService::get_album_by_id` and `get_tracks_by_album_id`

**Files:**
- Modify: `apps/server/src/services/library_service.rs`

- [ ] **Step 1: Add `get_album_by_id`**

  Open `apps/server/src/services/library_service.rs`. After the `get_albums` method, add:

  ```rust
  pub async fn get_album_by_id(&self, id: Uuid) -> AppResult<Option<AlbumWithRefs>> {
      let albums = AlbumRepository::find_by_ids(&self.pool, &[id]).await?;
      let Some(album) = albums.into_iter().next() else {
          return Ok(None);
      };

      let artist_ids: Vec<Uuid> = album.artist_id.into_iter().collect();
      let track_stats =
          AlbumRepository::get_track_stats_for_album_ids(&self.pool, &[album.id]).await?;

      let artists = ArtistRepository::find_by_ids(&self.pool, &artist_ids).await?;
      let artist = album
          .artist_id
          .and_then(|id| artists.into_iter().find(|a| a.id == id));

      let (track_count, total_duration_ms) = track_stats
          .into_iter()
          .find(|(album_id, _, _)| *album_id == id)
          .map(|(_, count, dur)| (count, dur))
          .unwrap_or((0, 0));

      Ok(Some(AlbumWithRefs {
          album,
          artist,
          track_count,
          total_duration_ms,
      }))
  }
  ```

- [ ] **Step 2: Add `get_tracks_by_album_id`**

  Immediately after `get_album_by_id`, add:

  ```rust
  pub async fn get_tracks_by_album_id(
      &self,
      album_id: Uuid,
  ) -> AppResult<Vec<TrackWithRefs>> {
      let tracks = TrackRepository::find_by_album_id(&self.pool, album_id).await?;
      self.attach_refs(tracks).await
  }
  ```

- [ ] **Step 3: Verify it compiles**

  ```bash
  cd apps/server && cargo check 2>&1 | grep -E "^error"
  ```

  Expected: no output.

- [ ] **Step 4: Commit**

  ```bash
  git add apps/server/src/services/library_service.rs
  git commit -m "feat: add LibraryService album detail and tracks methods"
  ```

---

## Task 3: Add album detail handlers and routes

**Files:**
- Modify: `apps/server/src/handlers/albums_handlers.rs`
- Modify: `apps/server/src/routes/album_routes.rs`

- [ ] **Step 1: Add imports to the handlers file**

  Open `apps/server/src/handlers/albums_handlers.rs`. Replace the existing imports block with:

  ```rust
  use crate::{
      dto::{
          request::PaginationParams,
          response::{
              album_response::AlbumResponse,
              track_response::TrackResponse,
              PaginatedResponse,
          },
      },
      error::{AppError, AppResult},
      services::LibraryService,
      state::AppState,
  };
  use axum::{
      extract::{Path, Query, State},
      Json,
  };
  use uuid::Uuid;
  ```

- [ ] **Step 2: Add `get_album` handler**

  After the existing `get_albums` function, add:

  ```rust
  pub async fn get_album(
      State(state): State<AppState>,
      Path(id): Path<Uuid>,
  ) -> AppResult<Json<AlbumResponse>> {
      let service = LibraryService::new(state.db.clone());
      let album = service
          .get_album_by_id(id)
          .await?
          .ok_or(AppError::NotFound)?;

      Ok(Json(album.into()))
  }
  ```

- [ ] **Step 3: Add `get_album_tracks` handler**

  After `get_album`, add:

  ```rust
  pub async fn get_album_tracks(
      State(state): State<AppState>,
      Path(id): Path<Uuid>,
  ) -> AppResult<Json<Vec<TrackResponse>>> {
      let service = LibraryService::new(state.db.clone());
      let tracks = service.get_tracks_by_album_id(id).await?;

      Ok(Json(tracks.into_iter().map(Into::into).collect()))
  }
  ```

- [ ] **Step 4: Register the new routes**

  Open `apps/server/src/routes/album_routes.rs`. Replace its full contents with:

  ```rust
  use crate::{handlers::albums_handlers, state::AppState};
  use axum::{routing::get, Router};

  pub fn albums_routes() -> Router<AppState> {
      Router::new()
          .route("/albums", get(albums_handlers::get_albums))
          .route("/albums/:id", get(albums_handlers::get_album))
          .route("/albums/:id/tracks", get(albums_handlers::get_album_tracks))
  }
  ```

- [ ] **Step 5: Build and verify**

  ```bash
  cd apps/server && cargo build 2>&1 | grep -E "^error"
  ```

  Expected: no output. A successful build means all three endpoints are wired up.

- [ ] **Step 6: Commit**

  ```bash
  git add apps/server/src/handlers/albums_handlers.rs apps/server/src/routes/album_routes.rs
  git commit -m "feat: add GET /albums/:id and GET /albums/:id/tracks endpoints"
  ```

---

## Task 4: Add `playNext` and `playLater` to `PlayerState`

**Files:**
- Modify: `apps/web/src/lib/player/player.svelte.ts`

- [ ] **Step 1: Add both methods**

  Open `apps/web/src/lib/player/player.svelte.ts`. After the `reorder` method (before the closing `}`), add:

  ```typescript
  playNext(tracks: TrackResponse[]) {
      if (tracks.length === 0) return;
      const insertAt = this.index + 1;
      this.queue = [
          ...this.queue.slice(0, insertAt),
          ...tracks,
          ...this.queue.slice(insertAt),
      ];
  }

  playLater(tracks: TrackResponse[]) {
      if (tracks.length === 0) return;
      this.queue = [...this.queue, ...tracks];
  }
  ```

- [ ] **Step 2: Verify TypeScript compiles**

  ```bash
  cd apps/web && bun run check 2>&1 | grep -E "error"
  ```

  Expected: no errors.

- [ ] **Step 3: Commit**

  ```bash
  git add apps/web/src/lib/player/player.svelte.ts
  git commit -m "feat: add playNext and playLater to PlayerState"
  ```

---

## Task 5: Create the server load for `/albums/[id]`

**Files:**
- Create: `apps/web/src/routes/(protected)/albums/[id]/+page.server.ts`

- [ ] **Step 1: Create the directory and file**

  Create `apps/web/src/routes/(protected)/albums/[id]/+page.server.ts` with:

  ```typescript
  import { error } from '@sveltejs/kit';
  import type { AlbumResponse } from '$lib/bindings/response/album/album-response';
  import type { TrackResponse } from '$lib/bindings/response/track/track-response';
  import type { PageServerLoad } from './$types';

  export const load: PageServerLoad = async ({ fetch, params }) => {
      const [albumRes, tracksRes] = await Promise.all([
          fetch(`/api/albums/${params.id}`),
          fetch(`/api/albums/${params.id}/tracks`),
      ]);

      if (albumRes.status === 404) {
          error(404, 'Album not found');
      }

      if (!albumRes.ok || !tracksRes.ok) {
          error(500, 'Failed to load album');
      }

      return {
          album: (await albumRes.json()) as AlbumResponse,
          tracks: (await tracksRes.json()) as TrackResponse[],
      };
  };
  ```

- [ ] **Step 2: Verify TypeScript compiles**

  ```bash
  cd apps/web && bun run check 2>&1 | grep -E "error"
  ```

  Expected: no errors.

- [ ] **Step 3: Commit**

  ```bash
  git add "apps/web/src/routes/(protected)/albums/[id]/+page.server.ts"
  git commit -m "feat: add server load for album detail page"
  ```

---

## Task 6: Create the album detail page

**Files:**
- Create: `apps/web/src/routes/(protected)/albums/[id]/+page.svelte`

- [ ] **Step 1: Create the page**

  Create `apps/web/src/routes/(protected)/albums/[id]/+page.svelte` with:

  ```svelte
  <script lang="ts">
      import { tracksColumns } from '$lib/components/tracks/columns.js';
      import DataTable from '$lib/components/ui/data-table/data-table.svelte';
      import { Button } from '$lib/components/ui/button';
      import { getPlayer } from '$lib/player/player.svelte.js';
      import { formatMilliseconds } from '$lib/utils/date.js';
      import PlayIcon from '@lucide/svelte/icons/play';
      import ShuffleIcon from '@lucide/svelte/icons/shuffle';
      import ListStartIcon from '@lucide/svelte/icons/list-start';
      import ListEndIcon from '@lucide/svelte/icons/list-end';
      import ListPlusIcon from '@lucide/svelte/icons/list-plus';
      import MusicIcon from '@lucide/svelte/icons/music';

      let { data } = $props();
      const player = getPlayer();

      const albumColumns = tracksColumns.filter((col) => col.id !== 'album');

      function play() {
          player.playQueue(data.tracks, 0);
      }

      function shuffle() {
          const shuffled = [...data.tracks].sort(() => Math.random() - 0.5);
          player.playQueue(shuffled, 0);
      }

      function playNext() {
          player.playNext(data.tracks);
      }

      function playLater() {
          player.playLater(data.tracks);
      }

      const year = data.album.date ? data.album.date.slice(0, 4) : null;
      const duration = formatMilliseconds(Number(data.album.totalDurationMs));
  </script>

  <div class="flex flex-col gap-6 p-6">
      <!-- Hero -->
      <div class="flex flex-row items-end gap-6">
          {#if data.album.coverUrl}
              <img
                  src={data.album.coverUrl}
                  alt={data.album.title}
                  class="size-48 shrink-0 rounded-lg object-cover shadow-lg"
              />
          {:else}
              <div
                  class="bg-muted text-muted-foreground flex size-48 shrink-0 items-center justify-center rounded-lg"
              >
                  <MusicIcon class="size-16 opacity-30" />
              </div>
          {/if}
          <div class="flex min-w-0 flex-col gap-1">
              <p class="text-muted-foreground text-xs font-medium uppercase tracking-widest">Album</p>
              <h1 class="truncate text-3xl font-bold">{data.album.title}</h1>
              {#if data.album.artist}
                  <a
                      href="/artists/{data.album.artist.id}"
                      class="text-muted-foreground hover:text-foreground w-fit text-sm transition-colors"
                  >
                      {data.album.artist.name}
                  </a>
              {/if}
              <p class="text-muted-foreground text-sm">
                  {[year, `${data.album.trackCount} tracks`, duration]
                      .filter(Boolean)
                      .join(' · ')}
              </p>
          </div>
      </div>

      <!-- Toolbar -->
      <div class="flex flex-row flex-wrap items-center gap-2">
          <Button onclick={play} class="gap-2">
              <PlayIcon class="size-4" />
              Play
          </Button>
          <Button variant="ghost" onclick={shuffle} class="gap-2">
              <ShuffleIcon class="size-4" />
              Shuffle
          </Button>
          <Button variant="ghost" onclick={playNext} class="gap-2">
              <ListStartIcon class="size-4" />
              Play Next
          </Button>
          <Button variant="ghost" onclick={playLater} class="gap-2">
              <ListEndIcon class="size-4" />
              Play Later
          </Button>
          <Button variant="ghost" disabled class="gap-2">
              <ListPlusIcon class="size-4" />
              Add to Playlist
          </Button>
      </div>

      <!-- Tracks -->
      <DataTable
          data={data.tracks}
          columns={albumColumns}
          onRowClick={(_, index) => player.playQueue(data.tracks, index)}
      />
  </div>
  ```

- [ ] **Step 2: Verify TypeScript compiles**

  ```bash
  cd apps/web && bun run check 2>&1 | grep -E "error"
  ```

  Expected: no errors.

- [ ] **Step 3: Commit**

  ```bash
  git add "apps/web/src/routes/(protected)/albums/[id]/+page.svelte"
  git commit -m "feat: album detail page with hero, toolbar, and tracks table"
  ```

---

## Verification Checklist

1. Navigate to `/albums` → click any row → confirm you land on `/albums/<id>` with no 404
2. Hero section shows album cover (or placeholder if none), title, artist, year, track count, duration
3. Artist name is a clickable link (href `/artists/<id>`)
4. **Play** — loads all album tracks into queue, starts playing from track 1
5. **Shuffle** — same tracks but in random order, starts playing
6. **Play Next** — with a track already playing, album tracks appear immediately after current track in queue
7. **Play Later** — album tracks appended to end of queue
8. **Add to Playlist** — button is visible but disabled/greyed out, not clickable
9. **Track row click** — starts playback from the clicked track's position
10. Unknown album ID → SvelteKit renders the 404 error page
