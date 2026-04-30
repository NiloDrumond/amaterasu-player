import type { TagResponse } from '$lib/bindings/response/tag/tag-response';
import type { TagSummaryResponse } from '$lib/bindings/response/tag/tag-summary-response';
import type { CreateTagParams } from '$lib/bindings/request/tag/create-tag-params';
import type { UpdateTagParams } from '$lib/bindings/request/tag/update-tag-params';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export function getTags(fetch: Fetch): Promise<Result<TagResponse[]>> {
	return api<TagResponse[]>(fetch, '/api/tags');
}

export function createTag(fetch: Fetch, params: CreateTagParams): Promise<Result<TagResponse>> {
	return api<TagResponse>(fetch, '/api/tags', { method: 'POST', body: params });
}

export function updateTag(
	fetch: Fetch,
	id: string,
	params: UpdateTagParams,
): Promise<Result<TagResponse>> {
	return api<TagResponse>(fetch, `/api/tags/${id}`, { method: 'PATCH', body: params });
}

export function deleteTag(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/tags/${id}`, { method: 'DELETE' });
}

export function getTrackTags(fetch: Fetch, trackId: string): Promise<Result<TagSummaryResponse[]>> {
	return api<TagSummaryResponse[]>(fetch, `/api/tracks/${trackId}/tags`);
}

export function attachTrackTag(
	fetch: Fetch,
	trackId: string,
	tagId: string,
): Promise<Result<void>> {
	return api<void>(fetch, `/api/tracks/${trackId}/tags/${tagId}`, { method: 'PUT' });
}

export function detachTrackTag(
	fetch: Fetch,
	trackId: string,
	tagId: string,
): Promise<Result<void>> {
	return api<void>(fetch, `/api/tracks/${trackId}/tags/${tagId}`, { method: 'DELETE' });
}

export function getAlbumTags(fetch: Fetch, albumId: string): Promise<Result<TagSummaryResponse[]>> {
	return api<TagSummaryResponse[]>(fetch, `/api/albums/${albumId}/tags`);
}

export function attachAlbumTag(
	fetch: Fetch,
	albumId: string,
	tagId: string,
): Promise<Result<void>> {
	return api<void>(fetch, `/api/albums/${albumId}/tags/${tagId}`, { method: 'PUT' });
}

export function detachAlbumTag(
	fetch: Fetch,
	albumId: string,
	tagId: string,
): Promise<Result<void>> {
	return api<void>(fetch, `/api/albums/${albumId}/tags/${tagId}`, { method: 'DELETE' });
}
