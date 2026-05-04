import type { TagCategoryResponse } from '$lib/bindings/response/tag-category/tag-category-response';
import type { CreateTagCategoryParams } from '$lib/bindings/request/tag-category/create-tag-category-params';
import type { UpdateTagCategoryParams } from '$lib/bindings/request/tag-category/update-tag-category-params';
import type { ReorderTagCategoriesParams } from '$lib/bindings/request/tag-category/reorder-tag-categories-params';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export function getTagCategories(fetch: Fetch): Promise<Result<TagCategoryResponse[]>> {
	return api<TagCategoryResponse[]>(fetch, '/api/tag-categories');
}

export function createTagCategory(
	fetch: Fetch,
	params: CreateTagCategoryParams,
): Promise<Result<TagCategoryResponse>> {
	return api<TagCategoryResponse>(fetch, '/api/tag-categories', {
		method: 'POST',
		body: params,
	});
}

export function updateTagCategory(
	fetch: Fetch,
	id: string,
	params: UpdateTagCategoryParams,
): Promise<Result<TagCategoryResponse>> {
	return api<TagCategoryResponse>(fetch, `/api/tag-categories/${id}`, {
		method: 'PATCH',
		body: params,
	});
}

export function deleteTagCategory(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/tag-categories/${id}`, { method: 'DELETE' });
}

export function reorderTagCategories(
	fetch: Fetch,
	params: ReorderTagCategoriesParams,
): Promise<Result<void>> {
	return api<void>(fetch, '/api/tag-categories/reorder', {
		method: 'PUT',
		body: params,
	});
}
