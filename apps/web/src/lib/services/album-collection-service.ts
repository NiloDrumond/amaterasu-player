import type { AlbumCollectionResponse } from '$lib/bindings/response/album-collection/album-collection-response';
import type { GetAlbumsResponse } from '$lib/bindings/response/album/get-albums-response';
import type { CreateAlbumCollectionParams } from '$lib/bindings/request/album-collection/create-album-collection-params';
import type { RenameAlbumCollectionParams } from '$lib/bindings/request/album-collection/rename-album-collection-params';
import type { UpdateAlbumCollectionFilterParams } from '$lib/bindings/request/album-collection/update-album-collection-filter-params';
import type { PaginationParams } from '$lib/bindings/request/common/pagination-params';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export function listCollections(fetch: Fetch): Promise<Result<AlbumCollectionResponse[]>> {
	return api<AlbumCollectionResponse[]>(fetch, '/api/collections');
}

export function getCollection(fetch: Fetch, id: string): Promise<Result<AlbumCollectionResponse>> {
	return api<AlbumCollectionResponse>(fetch, `/api/collections/${id}`);
}

export function createCollection(
	fetch: Fetch,
	params: CreateAlbumCollectionParams,
): Promise<Result<AlbumCollectionResponse>> {
	return api<AlbumCollectionResponse>(fetch, '/api/collections', {
		method: 'POST',
		body: params,
	});
}

export function renameCollection(
	fetch: Fetch,
	id: string,
	params: RenameAlbumCollectionParams,
): Promise<Result<AlbumCollectionResponse>> {
	return api<AlbumCollectionResponse>(fetch, `/api/collections/${id}`, {
		method: 'PATCH',
		body: params,
	});
}

export function updateCollectionFilter(
	fetch: Fetch,
	id: string,
	params: UpdateAlbumCollectionFilterParams,
): Promise<Result<AlbumCollectionResponse>> {
	return api<AlbumCollectionResponse>(fetch, `/api/collections/${id}/filter`, {
		method: 'PATCH',
		body: params,
	});
}

export function deleteCollection(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/collections/${id}`, { method: 'DELETE' });
}

export function getCollectionAlbums(
	fetch: Fetch,
	id: string,
	params?: PaginationParams,
): Promise<Result<GetAlbumsResponse>> {
	const query = params ? `?limit=${params.limit}&offset=${params.offset}` : '';
	return api<GetAlbumsResponse>(fetch, `/api/collections/${id}/albums${query}`);
}
