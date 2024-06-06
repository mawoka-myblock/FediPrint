import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { FullModelWithRelationsIds } from '$lib/helpers/typings';

export const load: PageLoad = async ({ fetch, url }) => {
	const page = parseInt(url.searchParams.get('p') ?? '0');
	if (isNaN(page)) {
		return error(400, 'page not a number');
	}
	const res = await fetch(`/api/v1/model/list?page=${page}`);
	if (res.ok) {
		const models: FullModelWithRelationsIds[] = await res.json();
		return { models };
	}
	return error(500, 'request failed with ' + res.status);
};
